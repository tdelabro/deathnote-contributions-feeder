use chrono::Utc;
use http_api_problem::HttpApiProblem;
use marketplace_domain::{
	Destination, Error as DomainError, ProjectEvent, Publisher, RandomUuidGenerator, UuidGenerator,
};
use marketplace_event_store::{bus::QUEUE_NAME, Event as StorableEvent, EventOrigin};
use marketplace_infrastructure::amqp::Bus;
use rocket::{response::status, serde::json::Json, State};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Deserialize;

use super::{to_http_api_problem::ToHttpApiProblem, uuid::UuidParam};

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Body {
	recipient_id: uuid::Uuid,
	requestor_id: uuid::Uuid,
	amount_in_usd: u32,
	reason: String,
}

#[openapi(tag = "Project")]
#[post(
	"/projects/<project_id>/payment-request",
	format = "application/json",
	data = "<body>"
)]
pub async fn request_payment(
	project_id: UuidParam,
	body: Json<Body>,
	event_publisher: &State<Bus>,
	uuid_generator: &State<RandomUuidGenerator>,
) -> Result<status::Accepted<String>, HttpApiProblem> {
	let body = body.into_inner();
	let payment_request_id = uuid_generator.new_uuid();

	let storable_event = StorableEvent {
		event: ProjectEvent::PaymentRequested {
			project_id: project_id.into(),
			id: payment_request_id,
			recipient_id: body.recipient_id,
			requestor_id: body.requestor_id,
			amount_in_usd: body.amount_in_usd,
			reason: body.reason,
		}
		.into(),
		deduplication_id: uuid_generator.new_uuid().to_string(),
		timestamp: Utc::now().naive_utc(),
		metadata: serde_json::Value::default(),
		origin: EventOrigin::BACKEND,
	};

	event_publisher
		.publish(
			Destination::Queue(String::from(QUEUE_NAME)),
			&storable_event,
		)
		.await
		.map_err(|e| DomainError::from(e).to_http_api_problem())?;

	Ok(status::Accepted(Some(payment_request_id.to_string())))
}
