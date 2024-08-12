#![allow(unused)]

use crate::schema::{company, company_position, follows, position, posts, users};
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::{
    prelude::{Associations, Identifiable},
    Insertable, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Default, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = company)]
pub struct Company {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
}

#[derive(
    Insertable, Queryable, Debug, Serialize, Deserialize, Default, Identifiable, Associations,
)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Position))]
#[diesel(belongs_to(Company))]
#[diesel(table_name = company_position)]
pub struct CompanyPosition {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub position_id: i64,
    pub company_id: i64,
}

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Default)]
#[diesel(primary_key(id))]
#[diesel(table_name = follows)]
pub struct Follow {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub following_user_id: i64,
    pub followed_user_id: i64,
}
#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Default, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = position)]
pub struct Position {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
}

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Default, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = posts)]
pub struct Post {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub body: String,
    pub user_id: i64,
}

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Default, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    #[diesel(deserialize_as = DateTime<Utc>)]
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
    pub username: String,
    pub profile_picture: Option<String>,
    pub password: String,
    pub company_position_id: Option<i64>,
    #[diesel(deserialize_as = i64)]
    pub role: i64,
}
