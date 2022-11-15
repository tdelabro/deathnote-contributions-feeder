use crate::{GithubIssue, GithubIssueNumber, GithubRepo, GithubRepoId, GithubUser, GithubUserId};
use async_trait::async_trait;
use mockall::automock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Infrastructure(anyhow::Error),
}

#[automock]
#[async_trait]
pub trait GithubClient: Send + Sync {
	async fn find_issue_by_id(
		&self,
		project_id: GithubRepoId,
		issue_number: GithubIssueNumber,
	) -> Result<GithubIssue, Error>;

	async fn find_repository_by_id(&self, project_id: GithubRepoId) -> Result<GithubRepo, Error>;

	async fn find_user_by_id(&self, user_id: GithubUserId) -> Result<GithubUser, Error>;

	async fn authenticate_user(&self, authorization_code: String) -> Result<GithubUserId, Error>;
}
