use crate::{
    db::DbPool,
    models::Company,
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
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Query {
    id: Option<i64>,
}

#[derive(Serialize)]
struct CompanyResult {
    id: i64,
    company_name: String,
    position_name: String,
}

#[get("")]
async fn get_company(req: HttpRequest, data: Data<DbPool>) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::company::dsl::*;
    use crate::schema::company_position::dsl::*;
    use crate::schema::position::dsl::*;

    use crate::schema::company::dsl::name as company_name;
    use crate::schema::company_position::dsl::id as company_position_id;
    use crate::schema::position::dsl::name as position_name;

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
        let result = company.find(i).first::<Company>(&mut connection);
        if let Ok(comp) = result {
            let comp: Company = comp;
            return Ok(OkResponse::new(
                "Company found".to_string(),
                Some(serde_json::to_value(comp).unwrap()),
            ));
        } else {
            return Err(ErrorResponse::new(
                StatusCode::NOT_FOUND,
                "Company not found".to_string(),
                Some("company_not_found".to_string()),
            ));
        }
    } else {
        let result = company_position
            .inner_join(company)
            .inner_join(position)
            .select((company_name, position_name, company_position_id))
            .load::<(String, String, i64)>(&mut connection);
        if let Ok(results) = result {
            let results: Vec<CompanyResult> = results
                .into_iter()
                .map(|(a, b, c)| CompanyResult {
                    company_name: a,
                    position_name: b,
                    id: c,
                })
                .collect();
            return Ok(OkResponse::new(
                "Companies found".to_string(),
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
struct CompanyForm {
    name: Option<Text<String>>,
}

#[post("")]
async fn add_company(
    MultipartForm(form): MultipartForm<CompanyForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::company::dsl::*;

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
    let company_name = match form.name {
        Some(n) => n.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Company name is required".to_string(),
                Some("company_name_required".to_string()),
            ));
        }
    };
    match diesel::insert_into(company)
        .values(Company {
            name: company_name,
            ..Default::default()
        })
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("Company added".to_string(), None)),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add company: {}", e),
            Some("company_add_failed".to_string()),
        )),
    }
}

#[derive(MultipartForm)]
struct CompanyUpdateForm {
    id: Option<Text<i64>>,
    name: Option<Text<String>>,
}

#[post("/update")]
async fn update(
    MultipartForm(form): MultipartForm<CompanyUpdateForm>,
    data: Data<DbPool>,
) -> Result<HttpResponse, ErrorResponse> {
    use crate::schema::company::dsl::*;

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
    let company_id = match form.id {
        Some(i) => i.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Company id is required".to_string(),
                Some("company_id_required".to_string()),
            ));
        }
    };
    let company_name = match form.name {
        Some(n) => n.into_inner(),
        None => {
            return Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "Company name is required".to_string(),
                Some("company_name_required".to_string()),
            ));
        }
    };
    match diesel::update(company.find(company_id))
        .set(name.eq(company_name))
        .execute(&mut connection)
    {
        Ok(_) => Ok(OkResponse::new("Company updated".to_string(), None)),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update company: {}", e),
            Some("company_update_failed".to_string()),
        )),
    }
}

pub fn init(config: &mut ServiceConfig) {
    config.service(
        web::scope("/company")
            .service(get_company)
            .service(add_company)
            .service(update),
    );
}
