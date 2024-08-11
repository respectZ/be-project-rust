use super::v1;
use actix_web::web::ServiceConfig;

pub fn init(config: &mut ServiceConfig) {
    v1::init(config);
}
