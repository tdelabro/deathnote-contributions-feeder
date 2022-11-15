use crate::{
	project::ProjectId,
	repositories::{ProjectRepository, ProjectRepositoryError},
};
use std::sync::Arc;

pub struct ProjectProjector {
	project_repository: Arc<dyn ProjectRepository>,
}

impl ProjectProjector {
	pub fn new(project_repository: Arc<dyn ProjectRepository>) -> Self {
		Self { project_repository }
	}

	pub fn on_create(&self, project_id: ProjectId) -> Result<(), ProjectRepositoryError> {
		self.project_repository.create(project_id)
	}

	pub fn on_payment_requested(
		&self,
		project_id: ProjectId,
	) -> Result<(), ProjectRepositoryError> {
		todo!();
	}
}
