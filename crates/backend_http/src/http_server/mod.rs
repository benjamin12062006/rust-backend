use std::sync::Arc;

use backend_application::user_ports::inbound::user_use_case::IUserUseCase;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply, http, path};

use crate::user_http::user_controller;
use crate::middleware;

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<dyn IUserUseCase>,
}

pub struct Server {
    port: u16,
    services: Services,
}

fn path_users_prefix() -> BoxedFilter<()> {
    path!("v1" / "users" / ..).boxed()
}

pub fn routes(
    services: Services,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
        ])
        .allow_methods(&[
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ]);

    let list_users = path_users_prefix()
        .and(path::end())
        .and(warp::get())
        .and(middleware::service::with_service(
            services.user_service.clone(),
        ))
        .and_then(user_controller::list_users);

    let create_user = path_users_prefix()
        .and(path::end())
        .and(warp::post())
        .and(middleware::service::with_service(
            services.user_service.clone(),
        ))
        .and(warp::body::json())
        .and_then(user_controller::create_user);

    let get_user = path_users_prefix()
        .and(warp::get())
        .and(middleware::service::with_service(services.user_service))
        .and(path::param::<Uuid>())
        .and(path::end())
        .and_then(user_controller::get_user);

    create_user.or(list_users).or(get_user).with(cors)
}

impl Server {
    pub fn new(port: u16, services: Services) -> Self {
        Self { port, services }
    }

    pub async fn run(&self) {
        let routes = routes(self.services.clone()).recover(middleware::error::handle_rejection);
        warp::serve(routes).run(([127, 0, 0, 1], self.port)).await;
    }
}
