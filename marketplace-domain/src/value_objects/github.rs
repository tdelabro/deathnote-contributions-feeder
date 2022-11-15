use url::Url;
pub type GithubRepoId = u64;
pub type IssueNumber = u64;
pub type UserId = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Issue {
	pub number: IssueNumber,
	pub project_id: GithubRepoId,
	pub title: String,
	pub description: Option<String>,
	pub external_link: Url,
	pub difficulty: Option<String>,
	pub technology: Option<String>,
	pub duration: Option<String>,
	pub context: Option<String>,
	pub r#type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Repository {
	pub project_id: GithubRepoId,
	pub owner: String,
	pub name: String,
	pub description: Option<String>,
	pub url: Option<Url>,
	pub logo_url: Option<Url>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct User {
	pub id: UserId,
	pub name: String,
}

impl Default for Issue {
	fn default() -> Self {
		Self {
			number: Default::default(),
			project_id: Default::default(),
			title: Default::default(),
			description: Default::default(),
			external_link: Url::parse("https://github.com/404").unwrap(),
			difficulty: Default::default(),
			technology: Default::default(),
			duration: Default::default(),
			context: Default::default(),
			r#type: Default::default(),
		}
	}
}
