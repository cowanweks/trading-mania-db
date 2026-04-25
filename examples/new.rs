use chrono::NaiveDate;

use db::prelude::{entities::user::Model, models::user::UserParams, util::connect_db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = connect_db("postgresql://user:password@localhost:5432/tradingmania", 20).await?;

    let new_user = UserParams {
        date_of_birth: NaiveDate::from_ymd_opt(2000, 06, 01).unwrap(),
        email: "cowannwekesa@gmail.com".to_string(),
        first_name: "Cowan".to_string(),
        gender: "Male".to_string(),
        last_name: "Wekesa".to_string(),
        middle_name: "Kanga".to_string(),
        password: "ultimate".to_string(),
        phone_no: "+254768676944".to_string(),
        ssid: "4dghdhdhdhdhdh".to_string(),
        username: "cowanweks".to_string(),
    };

    Model::create(&db, &new_user).await?;

    Ok(())
}
