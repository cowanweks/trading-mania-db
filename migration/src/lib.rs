pub mod entities;
pub mod models;
pub mod util;

mod m20220101_000001_create_table_user;
mod m20260416_152501_create_user_information_table;
mod m20260416_153510_create_broker_information_table;
mod m20260416_155923_create_address_table;
mod m20260416_160724_create_tradesignal_table;
mod m20260416_161733_create_trade_table;
mod m20260416_191747_create_tradesession_table;
mod m20260416_192431_create_backtestdata_table;
mod m20260418_173238_create_triggers;

pub(crate) use sea_orm_migration::prelude::*;

pub(crate) use m20220101_000001_create_table_user::User;
pub(crate) use m20260416_160724_create_tradesignal_table::TradeSignal;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_user::Migration),
            Box::new(m20260416_152501_create_user_information_table::Migration),
            Box::new(m20260416_153510_create_broker_information_table::Migration),
            Box::new(m20260416_155923_create_address_table::Migration),
            Box::new(m20260416_160724_create_tradesignal_table::Migration),
            Box::new(m20260416_161733_create_trade_table::Migration),
            Box::new(m20260416_191747_create_tradesession_table::Migration),
            Box::new(m20260416_192431_create_backtestdata_table::Migration),
            Box::new(m20260418_173238_create_triggers::Migration),
        ]
    }
}

pub mod prelude {
    pub use super::entities;
    pub use super::models;
    pub use super::util;

    pub mod enums {
        pub use super::entities::sea_orm_active_enums::TradeOutcome;
    }

    pub mod types {}
}
