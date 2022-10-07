use super::ContributionProjector;
use crate::*;
use mockall::predicate::*;
use rstest::*;
use std::sync::Arc;

#[fixture]
fn contribution_projection_repository() -> MockContributionProjectionRepository {
	MockContributionProjectionRepository::new()
}

#[fixture]
fn github_client() -> MockGithubClient {
	MockGithubClient::new()
}

#[fixture]
fn contribution_id() -> ContributionId {
	ContributionId::from(123)
}

#[fixture]
fn contributor_account_address() -> ContributorAccountAddress {
	123.into()
}

#[fixture]
fn project_id() -> GithubProjectId {
	123456
}

#[fixture]
fn issue_number() -> GithubIssueNumber {
	654321
}

#[fixture]
fn github_issue(project_id: GithubProjectId, issue_number: GithubIssueNumber) -> GithubIssue {
	GithubIssue {
		project_id,
		number: issue_number,
		..Default::default()
	}
}

#[fixture]
fn gate() -> u8 {
	2
}

#[fixture]
fn contribution(
	contribution_id: ContributionId,
	project_id: GithubProjectId,
	gate: u8,
	github_issue: GithubIssue,
	issue_number: GithubIssueNumber,
) -> ContributionProjection {
	ContributionProjection {
		id: contribution_id,
		project_id,
		issue_number,
		gate,
		contributor_account_address: None,
		status: ContributionStatus::Open,
		title: Some(github_issue.title),
		description: github_issue.description,
		external_link: Some(github_issue.external_link),
		metadata: ContributionProjectionMetadata {
			difficulty: github_issue.difficulty,
			technology: github_issue.technology,
			duration: github_issue.duration,
			context: github_issue.context,
			r#type: github_issue.r#type,
		},
	}
}

#[fixture]
fn contribution_created_event(
	contribution_id: ContributionId,
	project_id: GithubProjectId,
	issue_number: GithubIssueNumber,
	gate: u8,
) -> ContributionEvent {
	ContributionEvent::Created {
		id: contribution_id,
		project_id,
		issue_number,
		gate,
	}
}

#[fixture]
fn contribution_assigned_event(
	contribution_id: ContributionId,
	contributor_account_address: ContributorAccountAddress,
) -> ContributionEvent {
	ContributionEvent::Assigned {
		id: contribution_id,
		contributor_account_address,
	}
}

#[fixture]
fn contribution_unassigned_event(contribution_id: ContributionId) -> ContributionEvent {
	ContributionEvent::Unassigned {
		id: contribution_id,
	}
}

#[fixture]
fn contribution_validated_event(contribution_id: ContributionId) -> ContributionEvent {
	ContributionEvent::Validated {
		id: contribution_id,
	}
}

#[fixture]
fn new_gate() -> u8 {
	5
}

#[fixture]
fn gate_changed_event(contribution_id: ContributionId, new_gate: u8) -> ContributionEvent {
	ContributionEvent::GateChanged {
		id: contribution_id,
		gate: new_gate,
	}
}

#[rstest]
async fn on_contribution_created_event(
	mut contribution_projection_repository: MockContributionProjectionRepository,
	mut github_client: MockGithubClient,
	project_id: GithubProjectId,
	issue_number: GithubIssueNumber,
	github_issue: GithubIssue,
	contribution: ContributionProjection,
	contribution_created_event: ContributionEvent,
) {
	github_client
		.expect_find_issue_by_id()
		.with(eq(project_id), eq(issue_number))
		.returning(move |_, _| Ok(github_issue.clone()));

	contribution_projection_repository
		.expect_insert()
		.with(eq(contribution))
		.returning(|_| Ok(()));

	let projector = ContributionProjector::new(
		Arc::new(contribution_projection_repository),
		Arc::new(github_client),
	);

	projector.on_event(&Event::Contribution(contribution_created_event)).await;
}

#[rstest]
async fn on_contribution_assigned_event(
	mut contribution_projection_repository: MockContributionProjectionRepository,
	github_client: MockGithubClient,
	contributor_account_address: ContributorAccountAddress,
	contribution_id: ContributionId,
	contribution_assigned_event: ContributionEvent,
) {
	contribution_projection_repository
		.expect_update_contributor_and_status()
		.withf(
			move |input_contribution_id, input_contributor_id, input_status| {
				input_contribution_id.eq(&contribution_id)
					&& input_contributor_id.eq(&Some(&contributor_account_address))
					&& input_status.eq(&ContributionStatus::Assigned)
			},
		)
		.returning(|_, _, _| Ok(()));

	let projector = ContributionProjector::new(
		Arc::new(contribution_projection_repository),
		Arc::new(github_client),
	);

	projector.on_event(&Event::Contribution(contribution_assigned_event)).await;
}

#[rstest]
async fn on_contribution_unassigned_event(
	mut contribution_projection_repository: MockContributionProjectionRepository,
	github_client: MockGithubClient,
	contribution_id: ContributionId,
	contribution_unassigned_event: ContributionEvent,
) {
	contribution_projection_repository
		.expect_update_contributor_and_status()
		.withf(
			move |input_contribution_id, input_contributor_id, input_status| {
				input_contribution_id.eq(&contribution_id)
					&& input_contributor_id.eq(&None)
					&& input_status.eq(&ContributionStatus::Open)
			},
		)
		.returning(|_, _, _| Ok(()));

	let projector = ContributionProjector::new(
		Arc::new(contribution_projection_repository),
		Arc::new(github_client),
	);

	projector.on_event(&Event::Contribution(contribution_unassigned_event)).await;
}

#[rstest]
async fn on_contribution_validated_event(
	mut contribution_projection_repository: MockContributionProjectionRepository,
	github_client: MockGithubClient,
	contribution_id: ContributionId,
	contribution_validated_event: ContributionEvent,
) {
	contribution_projection_repository
		.expect_update_status()
		.with(eq(contribution_id), eq(ContributionStatus::Completed))
		.returning(|_, _| Ok(()));

	let projector = ContributionProjector::new(
		Arc::new(contribution_projection_repository),
		Arc::new(github_client),
	);

	projector.on_event(&Event::Contribution(contribution_validated_event)).await;
}

#[rstest]
async fn on_gate_changed_event(
	mut contribution_projection_repository: MockContributionProjectionRepository,
	github_client: MockGithubClient,
	contribution_id: ContributionId,
	new_gate: u8,
	gate_changed_event: ContributionEvent,
) {
	contribution_projection_repository
		.expect_update_gate()
		.with(eq(contribution_id), eq(new_gate))
		.returning(|_, _| Ok(()));

	let projector = ContributionProjector::new(
		Arc::new(contribution_projection_repository),
		Arc::new(github_client),
	);

	projector.on_event(&Event::Contribution(gate_changed_event)).await;
}
