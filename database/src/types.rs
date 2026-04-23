use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::Queryable,
    serialize::{self, IsNull, Output, ToSql},
};

use serde::{Deserialize, Serialize};

use std::io::Write;

use super::schema;

#[derive(Debug, Eq, PartialEq, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::external::sql_types::Broker)]
pub enum Broker {
    PocketOption,
    ExpertOption,
    Quotex,
}

impl ToSql<schema::external::sql_types::Broker, Pg> for Broker {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Broker::Quotex => out.write_all(b"Quotex")?,
            Broker::PocketOption => out.write_all(b"PocketOption")?,
            Broker::ExpertOption => out.write_all(b"ExpertOption")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::external::sql_types::Broker, Pg> for Broker {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Quotex" => Ok(Broker::Quotex),
            b"PocketOption" => Ok(Broker::PocketOption),
            b"ExpertOption" => Ok(Broker::ExpertOption),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::external::sql_types::TradeOutcome)]
pub enum TradeOutcome {
    Win,
    Loss,
}

impl ToSql<schema::external::sql_types::TradeOutcome, Pg> for TradeOutcome {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TradeOutcome::Win => out.write_all(b"WIN")?,
            TradeOutcome::Loss => out.write_all(b"LOSS")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::external::sql_types::TradeOutcome, Pg> for TradeOutcome {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"WIN" => Ok(TradeOutcome::Win),
            b"LOSS" => Ok(TradeOutcome::Loss),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::external::sql_types::TradeAction)]
pub enum TradeAction {
    Call,
    Put,
}

impl ToSql<schema::external::sql_types::TradeAction, Pg> for TradeAction {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TradeAction::Call => out.write_all(b"Call")?,
            TradeAction::Put => out.write_all(b"Put")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::external::sql_types::TradeAction, Pg> for TradeAction {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Call" => Ok(TradeAction::Call),
            b"Put" => Ok(TradeAction::Put),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::external::sql_types::TradeType)]
pub enum TradeType {
    Normal,
    Martingale,
}

impl ToSql<schema::external::sql_types::TradeType, Pg> for TradeType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TradeType::Normal => out.write_all(b"Normal")?,
            TradeType::Martingale => out.write_all(b"Martingale")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::external::sql_types::TradeType, Pg> for TradeType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Normal" => Ok(TradeType::Normal),
            b"Martingale" => Ok(TradeType::Martingale),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum CandleTimeFrame {
    S5 = 5,     // 5 seconds
    S10 = 10,   // 10 seconds
    S15 = 15,   // 15 seconds
    S30 = 30,   // 30 seconds
    M1 = 60,    // 1 minute(s)
    M2 = 120,   // 2 minute(s)
    M3 = 180,   // 3 minute(s)
    M5 = 300,   // 5 minute(s)
    M10 = 600,  // 10 minute(s)
    M15 = 900,  // 15 minute(s)
    M30 = 1800, // 30 minute(s)
    H1 = 3600,  // 1 hour(s)
    H4 = 14400, // 4 hour(s)
    D1 = 86400, // 1 Day(s)
}

impl CandleTimeFrame {
    pub fn to_seconds(&self) -> i32 {
        *self as i32
    }

    pub fn from_seconds(seconds: i32) -> Option<Self> {
        match seconds {
            5 => Some(CandleTimeFrame::S5),
            10 => Some(CandleTimeFrame::S10),
            15 => Some(CandleTimeFrame::S15),
            30 => Some(CandleTimeFrame::S30),
            60 => Some(CandleTimeFrame::M1),
            120 => Some(CandleTimeFrame::M2),
            180 => Some(CandleTimeFrame::M3),
            300 => Some(CandleTimeFrame::M5),
            600 => Some(CandleTimeFrame::M10),
            900 => Some(CandleTimeFrame::M15),
            1800 => Some(CandleTimeFrame::M30),
            3600 => Some(CandleTimeFrame::H1),
            14400 => Some(CandleTimeFrame::H4),
            86400 => Some(CandleTimeFrame::D1),
            _ => None,
        }
    }
}
