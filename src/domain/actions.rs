use crate::domain::{Contribution, ContributionId, ContributorId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
	CreateContribution {
		contribution: Box<Contribution>,
	},
	AssignContributor {
		contribution_id: ContributionId,
		contributor_id: ContributorId,
	},
	UnassignContributor {
		contribution_id: ContributionId,
	},
	ValidateContribution {
		contribution_id: ContributionId,
	},
}
