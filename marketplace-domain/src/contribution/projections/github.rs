use crate::{
	contribution::aggregate, ContributionId, ContributionStatus, ContributorAccountAddress,
	GithubIssueNumber, GithubRepoId, Projection,
};
use url::Url;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct GithubContribution {
	pub id: ContributionId,
	pub project_id: GithubRepoId,
	pub issue_number: GithubIssueNumber,
	pub contributor_account_address: Option<ContributorAccountAddress>,
	pub title: Option<String>,
	pub description: Option<String>,
	pub status: ContributionStatus,
	pub external_link: Option<Url>,
	pub gate: u8,
	pub metadata: Metadata,
	pub closed: bool,
}

impl Projection for GithubContribution {
	type A = aggregate::Contribution;
}

impl GithubContribution {
	pub fn old_composite_id(&self) -> u64 {
		let project_id: u64 = self.project_id;
		let issue_number = self.issue_number as u64;
		project_id * 1_000_000 + issue_number
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Metadata {
	pub difficulty: Option<String>,
	pub technology: Option<String>,
	pub duration: Option<String>,
	pub context: Option<String>,
	pub r#type: Option<String>,
}
