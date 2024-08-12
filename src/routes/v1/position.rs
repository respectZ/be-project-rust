use crate::{
    db::DbPool,
    models::Position,
    response::{ErrorResponse, OkResponse},
};
use actix_multipart::form::{text::Text, MultipartForm};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Data, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    id: Option<i64>,
}

#[get("")]
async fn get_position(req: HttpRequest, data: Data<DbPool>) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::position::dsl::*;

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
    let query = web::Query::<Query>::from_query(req.query_string()).unwrap();
    if let Some(i) = query.id {
        let result = position.find(i).first::<Position>(&mut connection);
        if let Ok(pos) = result {
            let pos: Position = pos;
            return Ok(OkResponse::new(
                "Position found".to_string(),
                Some(serde_json::to_value(pos).unwrap()),
            ));
        } else {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "Position not found".to_string(),
                Some("position_not_found".to_string()),
            ));
        }
    } else {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid query".to_string(),
            Some("invalid_query".to_string()),
        ));
    }
}

#[derive(Debug, MultipartForm)]
struct PositionForm {
    name: Option<Text<String>>,
}

#[post("")]
async fn add_position(
    MultipartForm(form): MultipartForm<PositionForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::position::dsl::*;

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
    let position_name = match form.name {
        Some(n) => n.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Position name is required".to_string(),
                Some("position_name_required".to_string()),
            ));
        }
    };
    match diesel::insert_into(position)
        .values(Position {
            name: position_name,
            ..Default::default()
        })
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("Position added".to_string(), None)),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add position: {}", e),
            Some("position_add_failed".to_string()),
        )),
    }
}

#[derive(MultipartForm)]
struct PositionUpdateForm {
    id: Option<Text<i64>>,
    name: Option<Text<String>>,
}

#[post("/update")]
async fn update(
    MultipartForm(form): MultipartForm<PositionUpdateForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::position::dsl::*;

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
    let position_id = match form.id {
        Some(i) => i.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Position id is required".to_string(),
                Some("position_id_required".to_string()),
            ));
        }
    };
    let position_name = match form.name {
        Some(n) => n.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Position name is required".to_string(),
                Some("position_name_required".to_string()),
            ));
        }
    };
    match diesel::update(position.find(position_id))
        .set(name.eq(position_name))
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("Position updated".to_string(), None)),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update position: {}", e),
            Some("position_update_failed".to_string()),
        )),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/position")
            .service(get_position)
            .service(add_position)
            .service(update),
    );
}
