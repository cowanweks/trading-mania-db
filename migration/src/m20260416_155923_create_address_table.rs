use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        uuid(Address::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(Address::Street))
                    .col(string(Address::City))
                    .col(string(Address::ZipCode))
                    .col(uuid(Address::UserId).not_null())
                    .col(
                        timestamp_with_time_zone(Address::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Address::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-address-userid")
                    .from(Address::Table, Address::UserId)
                    .to(crate::User::Table, crate::User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"address\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"address\"")
            .await?;

        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Address {
    Table,
    Id,
    Street,
    City,
    ZipCode,
    UserId,
    CreatedAt,
    UpdatedAt,
}
