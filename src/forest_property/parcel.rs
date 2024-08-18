use serde::{Deserialize, Serialize};
use crate::forest_property::stand::Stands;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcels {
    #[serde(rename = "Parcel")]
    pub parcel: Vec<Parcel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: i64,
    #[serde(rename = "Stands")]
    pub stands: Stands,
}