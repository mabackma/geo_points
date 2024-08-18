use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::forest_property::tree_stand_data::{TreeStandData, Operations, SpecialFeatures};
use crate::forest_property::geometry::PolygonGeometry;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stands {
    #[serde(rename = "Stand")]
    pub stand: Vec<Stand>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stand {
    #[serde(rename = "StandBasicData")]
    pub stand_basic_data: StandBasicData,
    #[serde(rename = "TreeStandData")]
    pub tree_stand_data: Option<TreeStandData>,
    #[serde(rename = "Operations")]
    pub operations: Option<Operations>,
    #[serde(rename = "SpecialFeatures")]
    pub special_features: Option<SpecialFeatures>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandBasicData {
    #[serde(rename = "ChangeState")]
    pub change_state: i64,
    #[serde(rename = "ChangeTime")]
    pub change_time: String,
    #[serde(rename = "CompleteState")]
    pub complete_state: i64,
    #[serde(rename = "StandNumber")]
    pub stand_number: i64,
    #[serde(rename = "StandNumberExtension")]
    pub stand_number_extension: Value,
    #[serde(rename = "MainGroup")]
    pub main_group: i64,
    #[serde(rename = "SubGroup")]
    pub sub_group: Option<i64>,
    #[serde(rename = "FertilityClass")]
    pub fertility_class: Option<i64>,
    #[serde(rename = "SoilType")]
    pub soil_type: Option<i64>,
    #[serde(rename = "DrainageState")]
    pub drainage_state: Option<i64>,
    #[serde(rename = "DevelopmentClass")]
    pub development_class: Option<Value>,
    #[serde(rename = "StandQuality")]
    pub stand_quality: Option<i64>,
    #[serde(rename = "MainTreeSpecies")]
    pub main_tree_species: Option<i64>,
    #[serde(rename = "Accessibility")]
    pub accessibility: Option<i64>,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: String,
    #[serde(rename = "Area")]
    pub area: f64,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "StandInfo")]
    pub stand_info: Option<String>,
    #[serde(rename = "AreaDecrease")]
    pub area_decrease: Option<f64>,
    #[serde(rename = "DitchingYear")]
    pub ditching_year: Option<i64>,
    #[serde(rename = "Identifiers")]
    pub identifiers: Option<Identifiers>,
    #[serde(rename = "CuttingRestriction")]
    pub cutting_restriction: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    #[serde(rename = "Identifier")]
    pub identifier: Identifier,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[serde(rename = "IdentifierType")]
    pub identifier_type: i64,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: i64,
}