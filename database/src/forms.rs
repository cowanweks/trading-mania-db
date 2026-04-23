use super::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = schema::external::trade_session)]
pub struct NewSession {
    pub(crate) open_time: NaiveDateTime,
    pub(crate) close_time: NaiveDateTime,
}
