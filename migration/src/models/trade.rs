use anyhow::{anyhow, Result};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::ModelError;
use crate::entities::trade;

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeParams {
    pub order_id: Uuid,
    pub stake_amount: Decimal,
    pub profit: Decimal,
    pub user_id: Uuid,
    pub trade_signal_id: Uuid,
}

impl trade::Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Self>> {
        let trade = trade::Entity::find()
            .filter(trade::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(trade)
    }

    pub async fn create(db: &DatabaseConnection, params: TradeParams) -> Result<Self> {
        if trade::Entity::find()
            .filter(trade::Column::OrderId.eq(params.order_id))
            .one(db)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists.into());
        }

        let new_trade = trade::ActiveModel {
            order_id: Set(params.order_id),
            profit: Set(params.profit),
            stake_amount: Set(params.stake_amount),
            user_id: Set(params.user_id),
            trade_signal_id: Set(params.trade_signal_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(new_trade)
    }

    pub async fn update(
        db: &DatabaseConnection,
        trade_id: Uuid,
        params: TradeParams,
    ) -> Result<Self> {
        let trade = trade::Entity::find()
            .filter(trade::Column::Id.eq(trade_id))
            .one(db)
            .await?;

        if trade.is_none() {
            return Err(anyhow!("The target trade does not exist"));
        }

        let old_trade: trade::ActiveModel = trade.unwrap().into();

        Ok(old_trade.update(db).await?)
    }
}
