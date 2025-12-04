use diesel::prelude::*;
use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: Option<bool>,
}