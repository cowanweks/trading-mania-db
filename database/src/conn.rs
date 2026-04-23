use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub fn connect_db(
    database_url: &str,
    max_connections: u32,
) -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn std::error::Error>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = match Pool::builder().max_size(max_connections).build(manager) {
        Ok(pool) => pool,

        Err(err) => {
            panic!("error connecting to database: {}!", err);
        }
    };

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    fn tes_connect_db() {
        dotenv().ok();

        let test_url = std::env::var("DATABASE_URL").unwrap();

        let pool = connect_db(&test_url, 20);

        assert_eq!(pool.is_ok(), true);
    }

    #[test]
    fn test_connect_db_with_wrong_credentials() {
        let test_url = "postgresql://devuser2:password@localhost/tradingmania";

        let pool = connect_db(test_url, 20);

        assert_eq!(pool.is_err(), true);
    }
}
