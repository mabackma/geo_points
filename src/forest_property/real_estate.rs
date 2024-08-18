use serde::{Deserialize, Serialize};
use crate::forest_property::parcel::Parcels;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: ForestPropertyData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForestPropertyData {
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealEstates {
    #[serde(rename = "RealEstate")]
    pub real_estate: RealEstate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: i64,
    #[serde(rename = "AreaNumber")]
    pub area_number: i64,
    #[serde(rename = "GroupNumber")]
    pub group_number: i64,
    #[serde(rename = "UnitNumber")]
    pub unit_number: i64,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: String,
    #[serde(rename = "Parcels")]
    pub parcels: Parcels,
}