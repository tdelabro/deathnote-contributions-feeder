use serde::{Deserialize, Serialize};
use std::fmt::Display;

type UserId = uuid::Uuid;
type ProjectId = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
	Created {
		project_id: ProjectId,
	},
	PaymentRequested {
		project_id: ProjectId,
		id: uuid::Uuid,
		recipient_id: UserId,
		requestor_id: UserId,
		amount_in_usd: u32,
		reason: String,
	},
}

impl Display for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			serde_json::to_string(&self).map_err(|_| std::fmt::Error)?
		)
	}
}
