use crate::{domain::*, infrastructure::database::models, Event};
use diesel::{dsl::exists, prelude::*};
use log::error;
use marketplace_domain::Event as DomainEvent;
use marketplace_infrastructure::database::{
	schema::{event_deduplications, event_deduplications::dsl, events, events::index},
	Client,
};
use serde_json::{to_value as to_json, Value as Json};

type Result<T> = std::result::Result<T, EventStoreError>;

// TODO: factorize with the one in marketplace-infrastructure
trait NamedAggregate {
	fn aggregate_name(&self) -> &str;
}

impl NamedAggregate for Event {
	fn aggregate_name(&self) -> &str {
		match self.event {
			DomainEvent::Project(_) => "PROJECT",
			DomainEvent::Contributor(_) => "CONTRIBUTOR",
			DomainEvent::Payment(_) => "PAYMENT",
		}
	}
}

impl EventStore for Client {
	fn append(&self, aggregate_id: &str, storable_event: Event) -> Result<()> {
		let connection = self.connection().map_err(|e| {
			error!("Failed to connect to database: {e}");
			EventStoreError::Connection(e.into())
		})?;

		let domain_event = storable_event.event.clone();
		let event = models::Event {
			timestamp: storable_event.timestamp,
			aggregate_name: storable_event.aggregate_name().to_owned(),
			aggregate_id: aggregate_id.to_owned(),
			payload: serialize_event(&domain_event)?,
			origin: storable_event.origin.to_string(),
			metadata: storable_event.metadata.clone(),
		};

		connection
			.transaction(|| {
				let already_exists: bool = diesel::select(exists(
					dsl::event_deduplications
						.filter(dsl::deduplication_id.eq(storable_event.deduplication_id.clone())),
				))
				.get_result(&*connection)?;

				if already_exists {
					error!(
						"Event with same deduplication_id ({}) already exists. This event will be ignored.",
						storable_event.deduplication_id
					);
					return Ok(());
				}

				let inserted_event_index: i32 = diesel::insert_into(events::table)
					.values(&event)
					.returning(index)
					.get_result(&*connection)?;

				let deduplication = models::EventDeduplication {
					deduplication_id: storable_event.deduplication_id.clone(),
					event_index: inserted_event_index,
				};

				diesel::insert_into(event_deduplications::table)
					.values(&deduplication)
					.execute(&*connection)?;

				Ok(())
			})
			.map_err(|e: diesel::result::Error| {
				error!("Failed to insert event(s) into database: {e}");
				EventStoreError::Append(e.into())
			})?;

		Ok(())
	}
}

fn serialize_event(event: &DomainEvent) -> Result<Json> {
	match event {
		DomainEvent::Project(event) => to_json(event),
		DomainEvent::Contributor(event) => to_json(event),
		DomainEvent::Payment(event) => to_json(event),
	}
	.map_err(|e| {
		error!("Failed to serialize event {event:?}: {e}");
		EventStoreError::InvalidEvent(e.into())
	})
}
