use std::sync::Arc;

use marketplace_domain::{Error as DomainError, GithubUserId, UserRepository};
use uuid::Uuid;

pub type ProjectId = Uuid;

#[async_trait]
pub trait Usecase: Send + Sync {
	async fn request_payment(
		&self,
		amount: u32,
		recipient_id: GithubUserId,
		reason: serde_json::Value,
		project_id: ProjectId,
	) -> Result<(), DomainError>;
}

pub struct RequestPayment {
	user_repository: Arc<dyn UserRepository>,
}

impl RequestPayment {
	pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
		Self { user_repository }
	}

	pub fn new_usecase_boxed(user_repository: Arc<dyn UserRepository>) -> Box<dyn Usecase> {
		Box::new(Self::new(user_repository))
	}
}

#[async_trait]
impl Usecase for RequestPayment {
	async fn request_payment(
		&self,
		amount: u32,
		recipient_id: GithubUserId,
		reason: serde_json::Value,
		project_id: ProjectId,
	) -> Result<(), DomainError> {
		todo!()
	}
}
