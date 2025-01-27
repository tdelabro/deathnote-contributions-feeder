use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{MessagePayload, PaymentEvent, ProjectEvent};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
	Project(ProjectEvent),
	Payment(PaymentEvent),
}

impl Display for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			serde_json::to_string(self).map_err(|_| std::fmt::Error)?
		)
	}
}

impl MessagePayload for Event {}

#[cfg(test)]
mod test {
	use assert_json_diff::assert_json_include;
	use serde_json::{json, Value};

	use super::*;
	use crate::{Amount, BlockchainNetwork, Currency, PaymentReceipt};

	#[test]
	fn display_event_as_json() {
		let event = Event::Payment(PaymentEvent::Processed {
			id: Default::default(),
			receipt_id: Default::default(),
			amount: Amount::new(
				"500.45".parse().unwrap(),
				Currency::Crypto("USDC".to_string()),
			),
			receipt: PaymentReceipt::OnChainPayment {
				network: BlockchainNetwork::Ethereum,
				recipient_address: Default::default(),
				transaction_hash: Default::default(),
			},
		});
		assert_json_include!(
			actual: serde_json::from_str::<Value>(&event.to_string()).unwrap(),
			expected: json!({ "Payment": { "Processed": {} } })
		);
	}
}
