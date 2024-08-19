use serde_json::Value;
use serde::{Deserialize, Serialize};

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
    pub tree_stratum: Vec<TreeStratum>,
}

impl TreeStrata {
    pub fn new(tree_stratum: Vec<TreeStratum>) -> Self {
        TreeStrata { tree_stratum }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStratum {
    #[serde(rename = "ChangeState")]
    pub change_state: i64,
    #[serde(rename = "StratumNumber")]
    pub stratum_number: i64,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: i64,
    #[serde(rename = "Storey")]
    pub storey: i64,
    #[serde(rename = "Age")]
    pub age: i64,
    #[serde(rename = "StemCount")]
    pub stem_count: Option<i64>,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: Option<f64>,
    #[serde(rename = "MeanHeight")]
    pub mean_height: f64,
    #[serde(rename = "DataSource")]
    pub data_source: i64,
    #[serde(rename = "BasalArea")]
    pub basal_area: Option<f64>,
    #[serde(rename = "SawLogPercent")]
    pub saw_log_percent: Option<f64>,
    #[serde(rename = "SawLogVolume")]
    pub saw_log_volume: Option<f64>,
    #[serde(rename = "VolumeGrowth")]
    pub volume_growth: Option<f64>,
    #[serde(rename = "Volume")]
    pub volume: Option<f64>,
    #[serde(rename = "PulpWoodVolume")]
    pub pulp_wood_volume: Option<f64>,
}

impl TreeStratum {
    pub fn new(
        change_state: i64,
        stratum_number: i64,
        tree_species: i64,
        storey: i64,
        age: i64,
        stem_count: i64,
        mean_diameter: f64,
        mean_height: f64,
        data_source: i64,
        basal_area: f64,
        saw_log_percent: f64,
        saw_log_volume: f64,
        volume_growth: f64,
        volume: f64,
        pulp_wood_volume: f64,
    ) -> Self {
        TreeStratum {
            change_state,
            stratum_number,
            tree_species,
            storey,
            age,
            stem_count: Some(stem_count),
            mean_diameter: Some(mean_diameter),
            mean_height,
            data_source,
            basal_area: Some(basal_area),
            saw_log_percent: Some(saw_log_percent),
            saw_log_volume: Some(saw_log_volume),
            volume_growth: Some(volume_growth),
            volume: Some(volume),
            pulp_wood_volume: Some(pulp_wood_volume),
        }
    }
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
