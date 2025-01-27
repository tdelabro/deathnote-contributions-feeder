use std::sync::Arc;

use derive_getters::Getters;
use domain::{AggregateRootRepository, Event, GithubUserId, Payment, Project, Publisher, UserId};
use infrastructure::{amqp::UniqueMessage, github};
use presentation::http::guards::OptionUserId;

use super::{Error, Result};
use crate::{
	application,
	domain::{ArePayoutSettingsValid, Permissions},
	infrastructure::{
		database::{
			PendingProjectLeaderInvitationsRepository, ProjectDetailsRepository, UserInfoRepository,
		},
		web3::ens,
	},
};

pub struct Context {
	pub caller_permissions: Box<dyn Permissions>,
	caller_info: OptionUserId,
	pub request_payment_usecase: application::payment::request::Usecase,
	pub process_payment_usecase: application::payment::process::Usecase,
	pub cancel_payment_usecase: application::payment::cancel::Usecase,
	pub create_project_usecase: application::project::create::Usecase,
	pub remove_project_leader_usecase: application::project::remove_leader::Usecase,
	pub invite_project_leader_usecase: application::project::invite_leader::Usecase,
	pub accept_project_leader_invitation_usecase:
		application::project::accept_leader_invitation::Usecase,
	pub update_project_github_repo_id_usecase: application::project::update_github_repo_id::Usecase,
	pub project_details_repository: ProjectDetailsRepository,
	pub update_user_info_usecase: application::user::update_profile_info::Usecase,
}

impl Context {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		caller_permissions: Box<dyn Permissions>,
		caller_info: OptionUserId,
		event_publisher: Arc<dyn Publisher<UniqueMessage<Event>>>,
		payment_repository: AggregateRootRepository<Payment>,
		project_repository: AggregateRootRepository<Project>,
		project_details_repository: ProjectDetailsRepository,
		pending_project_leader_invitations_repository: PendingProjectLeaderInvitationsRepository,
		user_info_repository: UserInfoRepository,
		github: Arc<github::Client>,
		ens: Arc<ens::Client>,
	) -> Self {
		Self {
			caller_permissions,
			caller_info,
			request_payment_usecase: application::payment::request::Usecase::new(
				event_publisher.to_owned(),
				project_repository.clone(),
			),
			process_payment_usecase: application::payment::process::Usecase::new(
				event_publisher.to_owned(),
				payment_repository.clone(),
			),
			cancel_payment_usecase: application::payment::cancel::Usecase::new(
				event_publisher.to_owned(),
				payment_repository,
			),
			create_project_usecase: application::project::create::Usecase::new(
				event_publisher.to_owned(),
				project_details_repository.clone(),
				github.clone(),
			),
			remove_project_leader_usecase: application::project::remove_leader::Usecase::new(
				event_publisher.to_owned(),
				project_repository.clone(),
			),
			invite_project_leader_usecase: application::project::invite_leader::Usecase::new(
				pending_project_leader_invitations_repository.clone(),
			),
			accept_project_leader_invitation_usecase:
				application::project::accept_leader_invitation::Usecase::new(
					event_publisher.to_owned(),
					pending_project_leader_invitations_repository,
					project_repository.clone(),
				),
			update_project_github_repo_id_usecase:
				application::project::update_github_repo_id::Usecase::new(
					event_publisher.to_owned(),
					project_repository,
					github,
				),
			project_details_repository,
			update_user_info_usecase: application::user::update_profile_info::Usecase::new(
				user_info_repository,
				ArePayoutSettingsValid::new(ens),
			),
		}
	}

	pub fn caller_info(&self) -> Result<CallerInfo> {
		let user_id =
			self.caller_info.user_id().map_err(|e| Error::NotAuthenticated(e.to_string()))?;

		let caller_info = CallerInfo {
			user_id,
			github_user_id: self
				.caller_info
				.github_user_id()
				.map_err(|e| Error::NotAuthenticated(e.to_string()))?,
		};
		Ok(caller_info)
	}
}

impl juniper::Context for Context {}

#[derive(Getters)]
pub struct CallerInfo {
	user_id: UserId,
	github_user_id: GithubUserId,
}
