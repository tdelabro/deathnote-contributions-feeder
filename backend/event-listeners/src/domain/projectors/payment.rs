use anyhow::Result;
use async_trait::async_trait;
use domain::{Event, PaymentEvent, SubscriberCallbackError};

use crate::{
	domain::{EventListener, Payment},
	infrastructure::database::PaymentRepository,
};

pub struct Projector {
	repository: PaymentRepository,
}

impl Projector {
	pub fn new(repository: PaymentRepository) -> Self {
		Self { repository }
	}
}

#[async_trait]
impl EventListener for Projector {
	async fn on_event(&self, event: &Event) -> Result<(), SubscriberCallbackError> {
		if let Event::Payment(PaymentEvent::Processed {
			id,
			receipt_id,
			amount,
			receipt,
		}) = event
		{
			self.repository.insert(&Payment::new(
				*receipt_id,
				*amount.amount(),
				amount.currency().to_string(),
				serde_json::to_value(receipt)
					.map_err(|e| SubscriberCallbackError::Discard(e.into()))?,
				(*id).into(),
			))?
		}
		Ok(())
	}
}
