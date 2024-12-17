

use bigdecimal::BigDecimal;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{default, string::ToString};
use strum_macros;

use super::{buy_order::BuyOrder, comment::Comment, payment_method::{PaymentMethod, PaymentMethodData}, user::UserType};



#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Chat {
    pub id: String,
    pub sender:String,
    pub receiver:String,
    pub message:String,
    pub image:String,
    pub created_at:String,
    pub updated_at:String,
    pub pair_id:String
}