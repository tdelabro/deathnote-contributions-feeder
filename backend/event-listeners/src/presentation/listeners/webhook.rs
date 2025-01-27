use std::env;

use anyhow::Result;
use async_trait::async_trait;
use domain::{Event, SubscriberCallbackError};
use olog::{error, info};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::json;
use tracing::instrument;
use url::Url;

use crate::domain::EventListener;

const WEBHOOK_TARGET_ENV_VAR: &str = "EVENT_WEBHOOK_TARGET";

struct WebhookEvent(Event);

impl WebhookEvent {
	pub fn new(event: Event) -> Self {
		Self(event)
	}
}

impl Serialize for WebhookEvent {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("Event", 3)?;
		let event_object = json!(self.0).as_object().expect("Event must be an object").clone();
		let aggregate_name = event_object
			.keys()
			.next()
			.expect("Event must have the aggregate name as first level key");
		let aggregate_event_object = event_object
			.values()
			.next()
			.expect("Event must have someting as the first level value")
			.as_object()
			.expect("Event must have an object as first level value");
		let event_name = aggregate_event_object
			.keys()
			.next()
			.expect("Event must have the event name as second level key");
		let payload = aggregate_event_object
			.values()
			.next()
			.expect("Event must have someting as the second level value");

		state.serialize_field("aggregate_name", aggregate_name)?;
		state.serialize_field("event_name", event_name)?;
		state.serialize_field("payload", payload)?;
		state.end()
	}
}

pub struct EventWebHook {
	web_client: reqwest::Client,
}

impl EventWebHook {
	pub fn new(client: reqwest::Client) -> Self {
		Self { web_client: client }
	}
}

#[derive(Debug)]
enum Error {
	EnvVarNotSet(env::VarError),
	InvalidEnvVar(url::ParseError),
	FailToSendRequest(reqwest::Error),
	RespondWithErrorStatusCode(reqwest::Error),
}

#[async_trait]
impl EventListener for EventWebHook {
	async fn on_event(&self, event: &Event) -> Result<(), SubscriberCallbackError> {
		match send_event_to_webhook(&self.web_client, event).await {
			Ok(()) => {},
			Err(e) => match e {
				Error::EnvVarNotSet(_) => info!(
					"Event webhook ignored: environment variable '{WEBHOOK_TARGET_ENV_VAR}' is not set"
				),
				Error::InvalidEnvVar(e) => error!(
					"Failed to parse environment variable '{WEBHOOK_TARGET_ENV_VAR}' content to Url: {e}"
				),
				Error::FailToSendRequest(e) => error!("Failed to send event to hook target: {e}"),
				Error::RespondWithErrorStatusCode(e) => {
					error!("WebHook target failed to process event: {e}")
				},
			},
		};
		Ok(())
	}
}

#[instrument(skip(client))]
async fn send_event_to_webhook(client: &reqwest::Client, event: &Event) -> Result<(), Error> {
	let env_var = std::env::var(WEBHOOK_TARGET_ENV_VAR).map_err(Error::EnvVarNotSet)?;
	let target = Url::parse(&env_var).map_err(Error::InvalidEnvVar)?;
	let res = client
		.post(target.clone())
		.json(&WebhookEvent::new(event.to_owned()))
		.send()
		.await
		.map_err(Error::FailToSendRequest)?;
	res.error_for_status().map_err(Error::RespondWithErrorStatusCode)?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use std::ffi::OsString;

	use assert_matches::assert_matches;
	use envtestkit::{
		lock::{lock_read, lock_test},
		set_env,
	};
	use mockito;
	use testing::fixtures::{
		self,
		payment::{recipient_address, transaction_hash},
	};

	use super::*;

	#[allow(clippy::await_holding_lock)]
	#[tokio::test]
	async fn env_variable_not_set() {
		let _lock = lock_read();

		let event: Event = fixtures::payment::events::payment_processed().into();

		assert_matches!(
			send_event_to_webhook(&reqwest::Client::new(), &event).await,
			Err(Error::EnvVarNotSet(_))
		);
	}

	#[allow(clippy::await_holding_lock)]
	#[tokio::test]
	async fn env_variable_invalid() {
		let _lock = lock_test();
		let _test = set_env(OsString::from(WEBHOOK_TARGET_ENV_VAR), "Some random junk");

		let event: Event = fixtures::payment::events::payment_processed().into();

		assert_matches!(
			send_event_to_webhook(&reqwest::Client::new(), &event).await,
			Err(Error::InvalidEnvVar(_))
		);
	}

	#[allow(clippy::await_holding_lock)]
	#[tokio::test]
	async fn http_call_fail() {
		let server_url = mockito::server_url();
		let _m = mockito::mock("POST", "/webhook").with_status(400).expect(1).create();

		let mut target_url = server_url.clone();
		target_url.push_str("/webhook");

		let _lock = lock_test();
		let _test = set_env(OsString::from(WEBHOOK_TARGET_ENV_VAR), &target_url);

		let event: Event = fixtures::payment::events::payment_processed().into();

		assert_matches!(
			send_event_to_webhook(&reqwest::Client::new(), &event).await,
			Err(Error::RespondWithErrorStatusCode(_))
		);

		assert!(_m.matched());
	}

	#[test]
	fn webhook_event_serialize() {
		let event: Event = fixtures::payment::events::payment_processed().into();

		let webhook_event = WebhookEvent::new(event);

		let json_value = serde_json::to_value(&webhook_event).unwrap();

		let recipient_address = recipient_address();
		let transaction_hash = transaction_hash();

		let expected_json_value = json!({
			"aggregate_name":"Payment",
			"event_name":"Processed",
			"payload": {
				"id":"abad1756-18ba-42e2-8cbf-83369cecfb38",
				"receipt_id":"b5db0b56-ab3e-4bd1-b9a2-6a3d41f35b8f",
				"amount": {
					"amount":"500.45",
					"currency": {"Crypto":"USDC"}
				},
				"receipt": {
					"OnChainPayment": {
						"network":"Ethereum",
						"recipient_address": recipient_address,
						"transaction_hash": transaction_hash,
					}
				}
			}
		});

		assert_eq!(json_value, expected_json_value);
	}
}
