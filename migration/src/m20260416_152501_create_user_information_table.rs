use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserInformation::Table)
                    .if_not_exists()
                    .col(
                        uuid(UserInformation::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(UserInformation::FirstName))
                    .col(string(UserInformation::MiddleName))
                    .col(string(UserInformation::LastName))
                    .col(string(UserInformation::PhoneNo).unique_key())
                    .col(string(UserInformation::Email).unique_key())
                    .col(string(UserInformation::Gender))
                    .col(boolean(UserInformation::CanTrade).default(false))
                    .col(date(UserInformation::DateOfBirth))
                    .col(uuid(UserInformation::UserId).unique_key().not_null())
                    .col(
                        timestamp_with_time_zone(UserInformation::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(UserInformation::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-brokerinfo-userid")
                    .from(UserInformation::Table, UserInformation::UserId)
                    .to(crate::User::Table, crate::User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"user_information\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"user_information\"",
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserInformation {
    Table,
    Id,
    FirstName,
    MiddleName,
    LastName,
    PhoneNo,
    Email,
    Gender,
    CanTrade,
    DateOfBirth,
    UserId,
    CreatedAt,
    UpdatedAt,
}
