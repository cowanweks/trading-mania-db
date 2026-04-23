use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveIden)]
enum TradeOutcome {
    #[sea_orm(iden = "trade_outcome")]
    TradeOutcome,
    #[sea_orm(iden = "WIN")]
    Win,
    #[sea_orm(iden = "LOSS")]
    Loss,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TradeOutcome::TradeOutcome)
                    .values([TradeOutcome::Win, TradeOutcome::Loss])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Trade::Table)
                    .if_not_exists()
                    .col(
                        uuid(Trade::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(uuid(Trade::OrderId))
                    .col(decimal(Trade::StakeAmount))
                    .col(decimal(Trade::Profit))
                    .col(
                        ColumnDef::new(Trade::Outcome)
                            .custom(TradeOutcome::TradeOutcome)
                            .not_null(),
                    )
                    .col(uuid(Trade::UserId).not_null())
                    .col(uuid(Trade::TradeSignalId).not_null())
                    .col(
                        timestamp_with_time_zone(Trade::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Trade::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-trade-userid")
                    .from(Trade::Table, Trade::UserId)
                    .to(crate::User::Table, crate::User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-trade-signalid")
                    .from(Trade::Table, Trade::TradeSignalId)
                    .to(crate::TradeSignal::Table, crate::TradeSignal::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"trade\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"trade\"")
            .await?;

        manager
            .drop_table(Table::drop().table(Trade::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TradeOutcome::TradeOutcome).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Trade {
    Table,
    Id,
    OrderId,
    StakeAmount,
    Profit,
    Outcome,
    UserId,
    TradeSignalId,
    CreatedAt,
    UpdatedAt,
}
