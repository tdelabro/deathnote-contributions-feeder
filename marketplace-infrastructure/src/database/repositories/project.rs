use crate::database::{models, schema::projects::dsl, Client, DatabaseError};
use anyhow::anyhow;
use diesel::{prelude::*, query_dsl::BelongingToDsl};
use itertools::Itertools;
use log::error;
use marketplace_domain::*;

impl ProjectRepository for Client {
	fn find_all_with_contributions(
		&self,
	) -> Result<Vec<ProjectWithContributions>, ProjectRepositoryError> {
		let connection = self.connection().map_err(ProjectRepositoryError::from)?;

		let projects: Vec<models::Project> =
			dsl::projects.load(&*connection).map_err(DatabaseError::from)?;

		let contributions: Vec<Vec<models::Contribution>> =
			models::Contribution::belonging_to(&projects)
				.load(&*connection)
				.map_err(DatabaseError::from)?
				.grouped_by(&projects);

		let result = projects
			.into_iter()
			.zip(contributions)
			.map(|(project, contributions)| ProjectWithContributions {
				project: project.into(),
				contributions: contributions.into_iter().map_into().collect(),
			})
			.collect_vec();

		Ok(result)
	}
}

impl From<models::Project> for Project {
	fn from(project: models::Project) -> Self {
		Self {
			id: project.id.parse().unwrap(),
			name: project.name,
			owner: project.owner,
		}
	}
}

impl From<ProjectProjection> for models::Project {
	fn from(project: ProjectProjection) -> Self {
		Self {
			id: project.id.to_string(),
			name: project.name,
			owner: project.owner,
			description: project.description,
			url: project.url.map(|url| url.to_string()),
			logo_url: project.logo_url.map(|url| url.to_string()),
		}
	}
}

impl From<models::Project> for ProjectProjection {
	fn from(project: models::Project) -> Self {
		Self {
			id: project.id.parse().unwrap(),
			name: project.name,
			owner: project.owner,
			description: project.description,
			url: project.url.map(|url| url.parse().unwrap()),
			logo_url: project.logo_url.map(|url| url.parse().unwrap()),
		}
	}
}

impl From<DatabaseError> for ProjectRepositoryError {
	fn from(error: DatabaseError) -> Self {
		use diesel::result::{DatabaseErrorKind, Error as DieselError};

		match error {
			DatabaseError::Transaction(DieselError::DatabaseError(
				DatabaseErrorKind::UniqueViolation,
				_,
			)) => Self::AlreadyExist(Box::new(error)),
			DatabaseError::Transaction(DieselError::NotFound) => Self::NotFound,
			_ => Self::Infrastructure(Box::new(error)),
		}
	}
}

impl ProjectProjectionRepository for Client {
	fn insert(&self, project: ProjectProjection) -> Result<(), ProjectProjectionRepositoryError> {
		let connection = self.connection().map_err(ProjectProjectionRepositoryError::from)?;
		let project = models::Project::from(project);
		diesel::insert_into(dsl::projects)
			.values(&project)
			.execute(&*connection)
			.map_err(|e| {
				error!("Failed to insert project {project:?}: {e}");
				DatabaseError::from(e)
			})?;

		Ok(())
	}

	fn find_by_id(
		&self,
		project_id: GithubRepoId,
	) -> Result<ProjectProjection, ProjectProjectionRepositoryError> {
		let connection = self.connection().map_err(ProjectProjectionRepositoryError::from)?;

		let project: models::Project = dsl::projects
			.find(project_id.to_string())
			.get_result(&*connection)
			.map_err(DatabaseError::from)?;

		Ok(project.into())
	}

	fn list(&self) -> Result<Vec<ProjectProjection>, ProjectProjectionRepositoryError> {
		let connection = self.connection().map_err(ProjectProjectionRepositoryError::from)?;

		let project: Vec<models::Project> =
			dsl::projects.load(&*connection).map_err(DatabaseError::from)?;

		Ok(project.into_iter().map_into().collect())
	}
}

impl ProjectionRepository<ProjectProjection> for Client {
	fn clear(&self) -> Result<(), ProjectionRepositoryError> {
		self.clear_table(dsl::projects)
			.map_err(|e| ProjectionRepositoryError::Infrastructure(e.into()))
	}
}

impl From<DatabaseError> for ProjectProjectionRepositoryError {
	fn from(error: DatabaseError) -> Self {
		match error {
			DatabaseError::Transaction(diesel::result::Error::DatabaseError(kind, _)) => match kind
			{
				diesel::result::DatabaseErrorKind::UniqueViolation =>
					Self::AlreadyExist(anyhow!(error)),
				_ => Self::Infrastructure(anyhow!(error)),
			},
			DatabaseError::Transaction(diesel::result::Error::NotFound) =>
				Self::NotFound(anyhow!(error)),
			_ => Self::Infrastructure(anyhow!(error)),
		}
	}
}
