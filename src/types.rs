use fake::faker::{
    internet::en::FreeEmail,
    name::en::{FirstName, LastName},
    phone_number::en::PhoneNumber,
};
use fake::{Dummy, Fake};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use ttl_cache::TtlCache;

#[derive(Dummy, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Information {
    pub pilot: Pilot,
    #[dummy(faker = "0.0")]
    pub distance: f64,
}

#[derive(Dummy, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pilot {
    #[dummy(faker = "FirstName()")]
    pub first_name: String,
    #[dummy(faker = "LastName()")]
    pub last_name: String,
    #[dummy(faker = "PhoneNumber()")]
    pub phone_number: String,
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
}

pub type SerialNumber = String;
pub type Cache = Arc<RwLock<TtlCache<SerialNumber, Information>>>;
