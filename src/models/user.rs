use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, JoinType, QuerySelect, RelationTrait, TransactionTrait};
use sea_orm::{ActiveValue::Set, Condition};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::error::ModelError;
use crate::entities::{broker_information, user, user_information};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserParams {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub phone_no: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BrokerParams {
    ssid: String,
}

impl user::Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Self>> {
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_credentials(
        db: &DatabaseConnection,
        identifier: &str,
    ) -> Result<Option<Self>> {
        // Query where email = identifier OR username = identifier
        let user = user::Entity::find()
            .find_also_related(user_information::Entity)
            .filter(
                Condition::any()
                    .add(user_information::Column::Email.eq(identifier))
                    .add(user::Column::UserName.eq(identifier)),
            )
            .one(db)
            .await?;

        Ok(user.map(|(user, _)| user))
    }

    pub async fn sign_in(db: &DatabaseConnection, params: &LoginParams) -> Result<()> {
        let user = Self::find_by_credentials(db, &params.identifier)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        // Verify password (assuming you have password hashing)
        if !user.verify_password(&user.password, &params.password)? {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        Ok(())
    }

    pub fn verify_password(&self, password_hash: &str, password: &str) -> Result<bool> {
        let argon2 = Argon2::default();

        let parsed_hash = PasswordHash::new(&password_hash).unwrap();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn create(db: &DatabaseConnection, params: &UserParams) -> Result<Self> {
        let txn = db.begin().await?;

        if user::Entity::find()
            .join(JoinType::InnerJoin, user::Relation::UserInformation.def())
            .filter(user_information::Column::Email.eq(&params.email))
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists.into());
        }

        let argon2 = Argon2::default();

        let salt = SaltString::generate(&mut OsRng);

        let password_hash = argon2.hash_password(params.password.as_bytes(), &salt)?;

        // New user
        let new_user = user::ActiveModel {
            user_name: Set(params.username.clone()),
            password: Set(password_hash.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let date_of_birth = NaiveDate::parse_from_str(&params.date_of_birth.clone(), "%d-%m-%Y");

        // New user information
        user_information::ActiveModel {
            user_id: Set(new_user.id),
            first_name: Set(params.first_name.clone()),
            last_name: Set(params.last_name.clone()),
            middle_name: Set(params.middle_name.clone()),
            email: Set(params.email.clone()),
            date_of_birth: Set(date_of_birth.unwrap()),
            gender: Set(params.gender.clone()),
            phone_no: Set(params.phone_no.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(new_user)
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: Uuid,
        params: UserParams,
    ) -> Result<Self> {
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(db)
            .await?;

        if user.is_none() {
            return Err(anyhow!("The target user does not exist"));
        }

        let old_user: user::ActiveModel = user.unwrap().into();

        Ok(old_user.update(db).await?)
    }

    pub async fn update_broker_info(
        db: &DatabaseConnection,
        user_id: Uuid,
        params: BrokerParams,
    ) -> Result<()> {
        let broker_info = broker_information::Entity::find()
            .filter(broker_information::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if broker_info.is_none() {
            return Err(anyhow!("The target user does not exist"));
        }

        let mut old_info: broker_information::ActiveModel = broker_info.unwrap().into();

        old_info.s_sid = Set(params.ssid);

        old_info.update(db).await?;

        Ok(())
    }

    pub async fn delete(&self, db: &DatabaseConnection, user_id: Uuid) -> Result<u64> {
        let result = user::Entity::delete_by_id(user_id).exec(db).await?;

        Ok(result.rows_affected)
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Self>> {
        let users = user::Entity::find().all(db).await?;

        Ok(users)
    }
}
