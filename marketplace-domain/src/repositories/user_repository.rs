use crate::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("User not found")]
	NotFound,
	#[error("Something happend at the infrastructure level")]
	Infrastructure(#[source] Box<dyn std::error::Error>),
}

pub trait Repository: Send + Sync {
	fn get_user_by_id(&self, user_id: GithubUserId) -> Result<(), Error>;
}
