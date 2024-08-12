use crate::{
    db::DbPool,
    models::User,
    response::{ErrorResponse, OkResponse},
    schema::users,
};
use actix_multipart::form::{text::Text, MultipartForm};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};
use diesel::{
    prelude::AsChangeset, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserQuery {
    email: Option<String>,
    username: Option<String>,
}

#[get("")]
async fn get_user(req: HttpRequest, data: Data<DbPool>) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::follows::dsl::{followed_user_id, follows};
    use crate::schema::users::dsl::*;

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
    let user_query = web::Query::<UserQuery>::from_query(req.query_string()).unwrap();
    let mut query = users.into_boxed();
    if let Some(u) = &user_query.username {
        query = query.filter(username.eq(u));
    } else if let Some(e) = &user_query.email {
        query = query.filter(email.eq(e));
    } else {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid query".to_string(),
            Some("invalid_query".to_string()),
        ));
    }
    let results: Vec<User> = query
        .load::<User>(&mut connection)
        .expect("Error loading users");
    if results.is_empty() {
        return Err(ErrorResponse::new(
            StatusCode::NOT_FOUND,
            "User not found".to_string(),
            Some("user_not_found".to_string()),
        ));
    }
    let uuser = &results[0];
    // Get follower count
    let follow_count = follows
        .filter(followed_user_id.eq(uuser.id.unwrap()))
        .count()
        .get_result::<i64>(&mut connection);
    let mut result = serde_json::to_value(uuser).unwrap();
    let result = result.as_object_mut().unwrap();
    result.insert("follow_count".to_string(), follow_count.unwrap().into());
    let value = serde_json::to_value(result).unwrap();
    Ok(OkResponse::new(
        "User found".to_string(),
        Some(vec![value].into()),
    ))
}

#[get("/add_dummy_user")]
async fn add_dummy_user(data: Data<DbPool>) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::users::dsl::*;
    let connection = &mut data.get().expect("Failed to get db connection from pool");
    let new_user = User {
        email: "ex@example.com".to_string(),
        username: "theuser1d".to_string(),
        profile_picture: None,
        password: "password".to_string(),
        ..Default::default()
    };
    match diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
    {
        Ok(_) => Ok(OkResponse::new(
            "User added".to_string(),
            Some(serde_json::to_value(new_user).unwrap()),
        )),
        Err(err) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user: {}", err),
            Some("add_user_failed".to_string()),
        )),
    }
}

#[derive(Deserialize, Debug)]
struct RegisterUser {
    email: String,
    username: String,
    password: String,
}

#[post("")]
async fn register(
    params: web::Form<RegisterUser>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::users::dsl::*;
    use diesel::OptionalExtension;

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
    // Check existing user by email
    let existing_user = users
        .filter(email.eq(&params.email).or(username.eq(&params.username)))
        .first::<User>(&mut connection)
        .optional();
    match existing_user {
        Ok(Some(u)) => {
            let u: User = u;
            if u.email == params.email {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    "Duplicate email".to_string(),
                    Some("duplicate_email".to_string()),
                ));
            }
            if u.username == params.username {
                return Err(ErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    "Duplicate username".to_string(),
                    Some("duplicate_username".to_string()),
                ));
            }
        }
        Err(err) => {
            return Err(ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to check existing user: {}", err),
                Some("check_user_failed".to_string()),
            ));
        }
        _ => {}
    }
    let new_user = User {
        email: params.email.clone(),
        username: params.username.clone(),
        password: params.password.clone(),
        ..Default::default()
    };
    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new(
            "User added".to_string(),
            Some(serde_json::to_value(new_user).unwrap()),
        )),
        Err(err) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user: {}", err),
            Some("add_user_failed".to_string()),
        )),
    }
}

#[derive(Debug, MultipartForm)]
struct UserForm {
    old_email: Text<String>,
    email: Option<Text<String>>,
    username: Option<Text<String>>,
    password: Option<Text<String>>,
    // #[multipart(limit = "10MB")]
    // file: Option<TempFile>,
}
#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct UserUpdate {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}
#[post("/update")]
async fn update_user(
    MultipartForm(form): MultipartForm<UserForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::users::dsl::*;
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
    let user_update = UserUpdate {
        email: match form.email {
            Some(e) => Some(e.into_inner()),
            None => None,
        },
        username: match form.username {
            Some(u) => Some(u.into_inner()),
            None => None,
        },
        password: match form.password {
            Some(p) => Some(p.into_inner()),
            None => None,
        },
    };
    match diesel::update(users)
        .filter(email.eq(form.old_email.into_inner()))
        .set(user_update)
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("User updated".to_string(), None)),
        Err(err) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update user: {}", err),
            Some("update_user_failed".to_string()),
        )),
    }
}

pub fn init(config: &mut ServiceConfig) {
    config.service(
        web::scope("/user")
            .service(get_user)
            .service(add_dummy_user)
            .service(register)
            .service(update_user),
    );
}
