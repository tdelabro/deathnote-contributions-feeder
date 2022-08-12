use std::sync::Arc;

use mapinto::ResultMapErrInto;

use crate::domain::{
	ApplicationId, ApplicationService, ContributionRepository, ContributionRepositoryError,
	DomainError, OnchainContributionService,
};

pub trait Usecase: Send + Sync {
	fn accept_application(&self, application_id: &ApplicationId) -> Result<(), DomainError>;
}

pub struct AcceptApplication {
	onchain_contribution_service: Arc<dyn OnchainContributionService>,
	contribution_repository: Arc<dyn ContributionRepository>,
	application_repository: Arc<dyn ApplicationService>,
}

impl AcceptApplication {
	pub fn new_usecase_boxed(
		onchain_contribution_service: Arc<dyn OnchainContributionService>,
		contribution_repository: Arc<dyn ContributionRepository>,
		application_repository: Arc<dyn ApplicationService>,
	) -> Box<dyn Usecase> {
		Box::new(Self {
			onchain_contribution_service,
			contribution_repository,
			application_repository,
		})
	}
}

impl Usecase for AcceptApplication {
	fn accept_application(&self, application_id: &ApplicationId) -> Result<(), DomainError> {
		let accepted_application = self
			.application_repository
			.accept_application(application_id)
			.map_err(DomainError::from)?;

		let contribution = self
			.contribution_repository
			.find_by_id(accepted_application.contribution_id())
			.map_err(DomainError::from)?
			.ok_or_else(|| DomainError::from(ContributionRepositoryError::NotFound))?;

		self.onchain_contribution_service
			.assign_contributor(
				contribution.onchain_id,
				*accepted_application.contributor_id(),
			)
			.map_err_into()
	}
}