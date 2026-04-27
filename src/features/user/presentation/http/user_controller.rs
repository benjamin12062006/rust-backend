use std::sync::Arc;

use uuid::Uuid;
use warp::reply::json;
use warp::{Rejection, Reply, reject};

use crate::features::user::application::dto::input::create_user_input::CreateUserInput;
use crate::features::user::ports::inbound::user_use_case::IUserUseCase;
use crate::features::user::presentation::http::dto::user_request_dto::CreateUserDto;
use crate::features::user::presentation::http::dto::user_response_dto::UserResponseDto;

pub async fn list_users(
    user_service: Arc<dyn IUserUseCase>
) -> Result<impl Reply, Rejection> 
{
    match user_service.list_users().await {
        Ok(users) => Ok(json(
            &users
                .into_iter()
                .map(UserResponseDto::from)
                .collect::<Vec<UserResponseDto>>(),
        )),
        Err(e) => Err(reject::custom(e)),
    }
}

pub async fn create_user(
    user_service: Arc<dyn IUserUseCase>,
    body: CreateUserDto
) -> Result<impl Reply, Rejection> 
{
  let input = CreateUserInput::try_from(body).map_err(reject::custom)?; 
  match user_service.create_user(input).await {
      Ok(created_user)=> Ok(json(&UserResponseDto::from(created_user))),
      Err(e)=> Err(reject::custom(e))
  }
}


pub async fn get_user(
    user_service: Arc<dyn IUserUseCase>,
    id: Uuid
) -> Result<impl Reply, Rejection> {
    match user_service.get_user(&id).await {
        Ok(user) => Ok(json(&UserResponseDto::from(user))),
        Err(e) => Err(reject::custom(e)),
    }
}