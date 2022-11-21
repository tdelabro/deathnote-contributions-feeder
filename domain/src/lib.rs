mod repositories;
use derive_more::{AsRef, Display, From, Into};
pub use repositories::*;

mod value_objects;
use serde::{Deserialize, Serialize};
pub use value_objects::*;

mod services;
pub use services::*;

mod event;
pub use event::Event;

mod event_store;
pub use event_store::{Error as EventStoreError, MockStore as MockEventStore, Store as EventStore};

mod aggregate;
pub use aggregate::{Aggregate, AggregateRoot, EventSourcable};

mod messaging;
pub use messaging::{
	Destination, Message, Publisher, PublisherError, Subscriber, SubscriberCallbackError,
	SubscriberError,
};

mod project;
pub use project::{Event as ProjectEvent, Id as ProjectId, Project};

mod aggregate_root_repository;
pub use aggregate_root_repository::{
	Error as AggregateRootRepositoryError, Repository as AggregateRootRepository,
};

mod payment;
pub use payment::{Event as PaymentEvent, Id as PaymentId, Payment, Receipt as PaymentReceipt};

mod payment_request;
pub use payment_request::{Id as PaymentRequestId, PaymentRequest};

#[derive(
	Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Display, From, Into, AsRef,
)]
pub struct UserId(uuid::Uuid);