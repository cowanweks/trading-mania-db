use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BrokerInformation::Table)
                    .if_not_exists()
                    .col(
                        uuid(BrokerInformation::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(BrokerInformation::SSid))
                    .col(boolean(BrokerInformation::IsDemo).default(false))
                    .col(
                        timestamp_with_time_zone(BrokerInformation::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(BrokerInformation::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(uuid(BrokerInformation::UserId).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-brokerinfo-userid")
                    .from(BrokerInformation::Table, BrokerInformation::UserId)
                    .to(crate::User::Table, crate::User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"broker_information\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"broker_information\"",
            )
            .await?;

        manager
            .drop_table(Table::drop().table(BrokerInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BrokerInformation {
    Table,
    Id,
    SSid,
    IsDemo,
    CreatedAt,
    UpdatedAt,
    UserId,
}
