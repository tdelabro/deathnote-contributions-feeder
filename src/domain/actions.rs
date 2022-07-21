use serde::{Deserialize, Serialize};

use crate::domain::{ContributionId, ContributorId, ProjectId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    CreateContribution {
        contribution_id: ContributionId,
        project_id: ProjectId,
        gate: u8,
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
