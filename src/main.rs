mod action_queue;
mod routes;

use diesel_migrations::*;
use log::info;
use rocket_okapi::{openapi_get_routes, swagger_ui::make_swagger_ui};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use tokio::{
    signal,
    sync::oneshot::{self, error::TryRecvError},
};

use deathnote_contributions_feeder::{
    database::{connections::pg_connection, run_db_migrations},
    github,
    utils::caches::{ContributorCache, IssueCache, RepoCache},
};

use dotenv::dotenv;
use rocket::routes;

use crate::action_queue::{execute_actions, ActionQueue};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    run_db_migrations();
    github::API::initialize();

    let action_queue = Arc::new(RwLock::new(ActionQueue::new()));

    // Allow to gracefully exit kill all thread on ctrl+c
    let (shutdown_send, mut shutdown_recv) = oneshot::channel();
    let ctr_c_handler = tokio::spawn(async {
        let _ = signal::ctrl_c().await;
        // ctrl_c have been pushed or something unexpected happened
        let _ = shutdown_send.send(true);
    });

    // Regularly create a transaction with tasks stored in the queue
    let cloned_action_queue = action_queue.clone();
    let queue_handler = tokio::spawn(async move {
        loop {
            let mut next_actions = vec![];
            if let Ok(mut queue) = cloned_action_queue.write() {
                next_actions = queue.pop_n(100);
            };
            if !next_actions.is_empty() {
                execute_actions(next_actions).await;
            }

            // Look if shutdown signat have been issued
            match shutdown_recv.try_recv() {
                Ok(_) => return,
                Err(TryRecvError::Closed) => return,
                Err(TryRecvError::Empty) => {}
            }
            // Wait a bit and do it again
            thread::sleep(Duration::from_secs(5));
        }
    });

    let rocket_handler = rocket::build()
        .manage(pg_connection::init_pool())
        .manage(action_queue.clone())
        .manage(IssueCache::default())
        .manage(RepoCache::default())
        .manage(ContributorCache::default())
        .attach(routes::cors::Cors)
        .mount(
            "/",
            routes![
                routes::cors::options_preflight_handler,
                routes::health::health_check,
            ],
        )
        .mount(
            "/",
            openapi_get_routes![
                routes::new_project,
                routes::list_projects,
                routes::create_contribution,
                routes::assign_contributor,
                routes::validate_contribution,
                routes::unassign_contributor,
            ],
        )
        .mount("/swagger", make_swagger_ui(&routes::get_docs()))
        .launch();

    let (rocket_result, ctr_c_result, queue_result) =
        tokio::join!(rocket_handler, ctr_c_handler, queue_handler);
    let _ = rocket_result.unwrap();
    queue_result.unwrap();
    ctr_c_result.unwrap();

    info!("Gracefully shut down");
}
