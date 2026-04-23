use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TradeSignal::Table)
                    .if_not_exists()
                    .col(
                        uuid(TradeSignal::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(TradeSignal::Asset))
                    .col(string(TradeSignal::Action))
                    .col(integer(TradeSignal::Expiry))
                    .col(timestamp_with_time_zone(TradeSignal::EntryTime))
                    .col(
                        timestamp_with_time_zone(TradeSignal::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(TradeSignal::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"trade_signal\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"trade_signal\"")
            .await?;

        manager
            .drop_table(Table::drop().table(TradeSignal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TradeSignal {
    Table,
    Id,
    Asset,
    EntryTime,
    Action,
    Expiry,
    CreatedAt,
    UpdatedAt,
}
