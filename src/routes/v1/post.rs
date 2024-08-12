use crate::{
    db::DbPool,
    models::{Post, User},
    response::{ErrorResponse, OkResponse},
    schema::users::name,
};
use actix_multipart::form::{text::Text, MultipartForm};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, ServiceConfig},
    HttpRequest, HttpResponse,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, Table};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PostQuery {
    username: Option<String>,
    id: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize)]
struct PostResult {
    post: Post,
    username: String,
    name: String,
}

#[get("")]
async fn get_posts(req: HttpRequest, data: Data<DbPool>) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::posts::dsl::*;
    use crate::schema::users::dsl::{id as uuser_id, username, users};

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
    let post_query = web::Query::<PostQuery>::from_query(req.query_string()).unwrap();
    let mut query = posts.into_boxed();
    let limit = post_query.limit.unwrap_or(20);
    let offset = post_query.offset.unwrap_or(0);
    if let Some(i) = post_query.id {
        // Find by id
        query = query.filter(id.eq(i));
        let results: Vec<Post> = query
            .load::<Post>(&mut connection)
            .expect("Error loading posts");
        if results.is_empty() {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "Post not found".to_string(),
                Some("post_not_found".to_string()),
            ));
        }
        let user = users
            .filter(uuser_id.eq(&results[0].user_id))
            .first::<crate::models::User>(&mut connection);
        if let Err(_) = user {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
        let user: crate::models::User = user.unwrap();
        let results: Vec<PostResult> = results
            .into_iter()
            .map(|p| PostResult {
                post: p,
                username: user.username.clone(),
                name: user.name.clone(),
            })
            .collect();
        return Ok(OkResponse::new(
            "Post found".to_string(),
            Some(serde_json::to_value(results).unwrap()),
        ));
    } else if let Some(u) = &post_query.username {
        // Find by username
        let user = users
            .filter(username.eq(u))
            .first::<crate::models::User>(&mut connection);
        if let Err(_) = user {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                Some("user_not_found".to_string()),
            ));
        }
        let user: crate::models::User = user.unwrap();
        let post_results = posts
            .filter(user_id.eq(user.id.unwrap()))
            .limit(limit)
            .offset(offset)
            .load::<Post>(&mut connection);
        if let Ok(ps) = post_results {
            let results: Vec<PostResult> = ps
                .into_iter()
                .map(|p| PostResult {
                    post: p,
                    username: u.clone(),
                    name: user.name.clone(),
                })
                .collect();
            return Ok(OkResponse::new(
                "Posts found".to_string(),
                Some(serde_json::to_value(results).unwrap()),
            ));
        } else {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "Posts not found".to_string(),
                Some("posts_not_found".to_string()),
            ));
        }
    } else {
        // Fetch all posts
        let post_results = posts
            .inner_join(users)
            .select((posts::all_columns(), username, name))
            .limit(limit)
            .offset(offset)
            .load::<(Post, String, String)>(&mut connection);
        if let Ok(results) = post_results {
            let results: Vec<PostResult> = results
                .into_iter()
                .map(|(p, u, n)| PostResult {
                    post: p,
                    username: u,
                    name: n,
                })
                .collect();
            if results.is_empty() {
                return Err(ErrorResponse::new(
                    StatusCode::NOT_FOUND,
                    "Posts not found".to_string(),
                    Some("posts_not_found".to_string()),
                ));
            }
            return Ok(OkResponse::new(
                "Posts found".to_string(),
                Some(serde_json::to_value(results).unwrap()),
            ));
        }
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid query".to_string(),
            Some("invalid_query".to_string()),
        ));
    }
}

#[derive(Debug, MultipartForm)]
struct PostForm {
    user_id: Option<Text<i64>>,
    body: Option<Text<String>>,
}

#[post("")]
async fn add_post(
    MultipartForm(form): MultipartForm<PostForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::posts::dsl::posts;
    use crate::schema::users::dsl::users;

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
    let user_id = match form.user_id {
        Some(u) => u.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "User id is required".to_string(),
                Some("user_id_required".to_string()),
            ));
        }
    };
    let body = match form.body {
        Some(b) => b.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Post body is required".to_string(),
                Some("post_body_required".to_string()),
            ));
        }
    };
    let user = users.find(user_id).first::<User>(&mut connection);
    if let Err(_) = user {
        return Err(ErrorResponse::new(
            StatusCode::NOT_FOUND,
            "User not found".to_string(),
            Some("user_not_found".to_string()),
        ));
    }
    let user: User = user.unwrap();
    match diesel::insert_into(posts)
        .values(Post {
            user_id: user.id.unwrap(),
            body,
            ..Default::default()
        })
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("Post added".to_string(), None)),
        Err(e) => {
            return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to add post: {}", e),
                Some("add_post_failed".to_string()),
            ));
        }
    }
}

// TODO: Edit post

pub fn init(config: &mut ServiceConfig) {
    config.service(web::scope("/post").service(get_posts).service(add_post));
}
