use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create set_updated_at() FIRST (the trigger function)
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION set_updated_at()
                RETURNS TRIGGER AS $$
                BEGIN
                    IF (
                        NEW IS DISTINCT FROM OLD
                        AND NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
                    ) THEN
                        NEW.updated_at := CURRENT_TIMESTAMP;
                    END IF;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // 2. Create manage_updated_at() SECOND (references set_updated_at)
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION manage_updated_at(_tbl REGCLASS)
                RETURNS VOID AS $$
                BEGIN
                    EXECUTE format(
                        'CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                         FOR EACH ROW EXECUTE PROCEDURE set_updated_at()',
                        _tbl
                    );
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // 3. Create the user table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        uuid(User::Id)
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(User::UserName).unique_key().not_null())
                    .col(string(User::Password).not_null())
                    .col(boolean(User::IsActive).default(true))
                    .col(
                        timestamp_with_time_zone(User::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(User::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Add the manage_updated_at trigger to the user table
        manager
            .get_connection()
            .execute_unprepared("SELECT manage_updated_at('\"user\"'::REGCLASS)")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Drop the updated_at trigger from the user table
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS set_updated_at_trigger ON \"user\"")
            .await?;

        // 2. Drop user table
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // 3. Drop manage_updated_at trigger function
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS manage_updated_at(REGCLASS) CASCADE")
            .await?;

        // 4. Drop set_updated_at trigger function
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS set_updated_at() CASCADE")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    UserName,
    Password,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
