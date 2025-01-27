use std::sync::Arc;

use derive_more::Constructor;
use domain::{Project, User};
use infrastructure::database::{schema::project_leads::dsl, Client};

#[derive(DieselMappingRepository, Constructor)]
#[entities((Project, User))]
#[ids((dsl::project_id, dsl::user_id))]
#[table(dsl::project_leads)]
pub struct Repository(Arc<Client>);
