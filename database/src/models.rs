use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema;
use super::types::{Broker, CandleTimeFrame, TradeAction, TradeOutcome, TradeType};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::user_information)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInformation {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub email: String,
    #[diesel(sql_type = TradeAction)]
    pub gender: String,
    pub can_trade: bool,
    pub middle_name: Option<String>,
    pub date_of_birth: NaiveDate,

    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::broker_information)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrokerInformation {
    pub id: Uuid,
    pub po_ssid: String,
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    pub account_balance: BigDecimal,
    pub po_user_id: i32,
    pub is_demo: bool,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub user_id: Uuid,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Address {
    pub id: Uuid,
    pub street: String,
    pub city: String,
    pub zip_code: String,

    pub user_id: Uuid,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::trade_signals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeSignal {
    pub id: Uuid,
    pub asset: String,
    pub entry_time: DateTime<Utc>,
    pub action: TradeAction,
    pub broker: Broker,
    pub expiry: i32,

    // pub session_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::trades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trade {
    pub id: Uuid,
    pub trade_type: TradeType,
    pub order_id: Uuid,
    pub asset: String,
    pub payout_rate: f64,
    pub entry_time: DateTime<Utc>,
    pub profit: BigDecimal,
    pub stake_amount: BigDecimal,

    pub action: TradeAction,
    pub outcome: Option<TradeOutcome>,

    pub user_id: Uuid,
    pub signal_id: Uuid,
    pub mart_trade_id: Option<Uuid>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::trade_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeSession {
    pub id: Uuid,
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::external::candles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Candle {
    pub id: Uuid,
    pub asset: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub timestamp: DateTime<Utc>,
    pub time_frame: i32,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
