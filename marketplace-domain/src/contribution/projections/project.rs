use crate::{Contribution, GithubRepoId, Projection};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Project {
	pub id: GithubRepoId,
	pub owner: String,
	pub name: String,
	pub url: Option<Url>,
	pub description: Option<String>,
	pub logo_url: Option<Url>,
}

impl Projection for Project {
	type A = Contribution;
}
