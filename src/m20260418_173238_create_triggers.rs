use sea_orm_migration::prelude::*;

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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop in reverse order
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS manage_updated_at(REGCLASS) CASCADE")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS set_updated_at() CASCADE")
            .await?;

        Ok(())
    }
}
