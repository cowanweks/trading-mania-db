use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BacktestData::Table)
                    .if_not_exists()
                    .col(
                        uuid(BacktestData::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(BacktestData::Asset))
                    .col(decimal(BacktestData::Open))
                    .col(decimal(BacktestData::High))
                    .col(decimal(BacktestData::Low))
                    .col(decimal(BacktestData::Close))
                    .col(integer(BacktestData::TimeFrame))
                    .col(timestamp_with_time_zone(BacktestData::Timestamp))
                    .col(
                        timestamp_with_time_zone(BacktestData::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(BacktestData::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"backtest_data\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"backtest_data\"",
            )
            .await?;

        manager
            .drop_table(Table::drop().table(BacktestData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BacktestData {
    Table,
    Id,
    Asset,
    Open,
    High,
    Low,
    Close,
    Timestamp,
    TimeFrame,
    CreatedAt,
    UpdatedAt,
}
