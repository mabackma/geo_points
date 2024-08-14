use serde_json::Value;
use serde::{Deserialize, Serialize};

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
pub struct PolygonGeometry {
    pub point_property: PointProperty,
    pub polygon_property: PolygonProperty,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointProperty {
    #[serde(rename = "Point")]
    pub point: Point,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub coordinates: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolygonProperty {
    #[serde(rename = "Polygon")]
    pub polygon: Polygon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polygon {
    pub exterior: Exterior,
    pub interior: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exterior {
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearRing {
    pub coordinates: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStandData {
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStandDataDate {
    #[serde(rename = "TreeStrata")]
    pub tree_strata: TreeStrata,
    #[serde(rename = "TreeStandSummary")]
    pub tree_stand_summary: Option<TreeStandSummary>,
    #[serde(rename = "DeadTreeStrata")]
    pub dead_tree_strata: Option<DeadTreeStrata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStrata {
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStandSummary {
    #[serde(rename = "ChangeState")]
    pub change_state: i64,
    #[serde(rename = "MeanAge")]
    pub mean_age: i64,
    #[serde(rename = "BasalArea")]
    pub basal_area: f64,
    #[serde(rename = "StemCount")]
    pub stem_count: i64,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: f64,
    #[serde(rename = "MeanHeight")]
    pub mean_height: f64,
    #[serde(rename = "Volume")]
    pub volume: f64,
    #[serde(rename = "VolumeGrowth")]
    pub volume_growth: f64,
    #[serde(rename = "Value")]
    pub value: Option<f64>,
    #[serde(rename = "ValueGrowthPercent")]
    pub value_growth_percent: Option<f64>,
    #[serde(rename = "SawLogVolume")]
    pub saw_log_volume: Option<f64>,
    #[serde(rename = "PulpWoodVolume")]
    pub pulp_wood_volume: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadTreeStrata {
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    #[serde(rename = "Operation")]
    pub operation: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialFeatures {
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Value,
}
