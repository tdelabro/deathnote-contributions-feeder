use crate::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Project not found")]
	NotFound,
	#[error("Project already exist")]
	AlreadyExist(#[source] Box<dyn std::error::Error>),
	#[error("Project contains invalid members")]
	InvalidEntity(#[source] Box<dyn std::error::Error>),
	#[error("Something happend at the infrastructure level")]
	Infrastructure(#[source] Box<dyn std::error::Error>),
}

pub trait Repository: Send + Sync {
	fn create(&self, id: ProjectId) -> Result<ProjectId, Error>;
	fn find(&self, id: ProjectId) -> Result<Project, Error>;
}
