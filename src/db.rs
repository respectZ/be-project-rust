use actix_web::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, PooledConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooled = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<DbPool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    Ok(pool)
}
