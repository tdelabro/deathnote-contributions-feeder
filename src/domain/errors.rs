use super::{
	ApplicationRepositoryError, ApplicationServiceError, ContributionRepositoryError,
	ContributionServiceError, ContributorRepositoryError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Application repository error")]
	ApplicationRepository(#[from] ApplicationRepositoryError),
	#[error("Contribution repository error")]
	ContributionRepository(#[from] ContributionRepositoryError),
	#[error("Contributor repository error")]
	ContributorRepository(#[from] ContributorRepositoryError),
	#[error("Contribution service error")]
	ContributionService(#[from] ContributionServiceError),
	#[error("Application service error")]
	ApplicationService(#[from] ApplicationServiceError),
	#[error("Failed to take control of a lock")]
	Lock,
}
