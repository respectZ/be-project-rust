use super::{company, position, user};
use actix_web::web::{self, ServiceConfig};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(user::init)
            .configure(company::init)
            .configure(position::init),
    );
}
