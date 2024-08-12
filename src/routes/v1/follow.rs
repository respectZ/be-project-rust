use actix_web::{
    http::StatusCode,
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Result,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{db::DbPool, models::User, response::ErrorResponse};

#[derive(Deserialize, Debug)]
struct FollowForm {
    user_id: i64,
    followed_user_id: i64,
}

#[post("/follow")]
async fn follow(
    form: web::Json<FollowForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::follows::dsl::*;
    use crate::schema::users::dsl::*;

    if form.user_id == form.followed_user_id {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "User cannot follow themselves".to_string(),
            Some("user_following_self".to_string()),
        ));
    }

    let mut connection = match data.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get db connection from pool: {}", e),
                Some("db_connection_failed".to_string()),
            ));
        }
    };
    let following_user = match users.find(form.user_id).first::<User>(&mut connection) {
        Ok(u) => {
            let u: User = u;
            u
        }
        Err(_) => {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
    };
    let followed_user = match users
        .find(form.followed_user_id)
        .first::<User>(&mut connection)
    {
        Ok(u) => {
            let u: User = u;
            u
        }
        Err(_) => {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
    };
    diesel::insert_into(follows)
        .values((
            following_user_id.eq(following_user.id.unwrap()),
            followed_user_id.eq(followed_user.id.unwrap()),
        ))
        .execute(&mut connection)
        .map_err(|e| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to follow user: {}", e),
                Some("follow_user_failed".to_string()),
            )
        })?;
    Ok(HttpResponse::Ok().finish())
}

#[post("/unfollow")]
async fn unfollow(
    form: web::Json<FollowForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::follows::dsl::*;
    use crate::schema::users::dsl::*;

    if form.user_id == form.followed_user_id {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "User cannot unfollow themselves".to_string(),
            Some("user_unfollowing_self".to_string()),
        ));
    }

    let mut connection = match data.get() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get db connection from pool: {}", e),
                Some("db_connection_failed".to_string()),
            ));
        }
    };
    let following_user = match users.find(form.user_id).first::<User>(&mut connection) {
        Ok(u) => {
            let u: User = u;
            u
        }
        Err(_) => {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
    };
    let followed_user = match users
        .find(form.followed_user_id)
        .first::<User>(&mut connection)
    {
        Ok(u) => {
            let u: User = u;
            u
        }
        Err(_) => {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
    };
    diesel::delete(
        follows
            .filter(following_user_id.eq(following_user.id.unwrap()))
            .filter(followed_user_id.eq(followed_user.id.unwrap())),
    )
    .execute(&mut connection)
    .map_err(|e| {
        ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to unfollow user: {}", e),
            Some("unfollow_user_failed".to_string()),
        )
    })?;
    Ok(HttpResponse::Ok().finish())
}

pub fn init(config: &mut ServiceConfig) {
    config.service(follow).service(unfollow);
}
