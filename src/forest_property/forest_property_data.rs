use core::f64;
use std::fs::{self, File};
use serde::{Deserialize, Serialize};
use super::{geometry::PolygonGeometry, stand::{Stand, Stands}};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ForestPropertyData {
    #[serde(rename = "RealEstates")]
    pub real_estates: RealEstates,
}

pub fn read_number_cli(min: usize, max: usize) -> usize {
    let mut buf = String::new();

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    let number: usize = buf.trim().parse().expect("Please type a number!");

    if number > max || number < min {
        panic!("Chosen number out of range")
    }

    number
}

impl ForestPropertyData {
    pub fn from_xml_file(path: &str) -> ForestPropertyData {
        let xml = fs::read_to_string(path).expect("Could not read the XML file");
        ForestPropertyData::parse_from_str(xml.as_str())
    }

    #[cfg(test)]
    pub fn from_json_file(path: &str) -> ForestPropertyData {
        let json = fs::read_to_string(path).expect("Could not read the JSON file");

        
        let property : ForestPropertyData = serde_json::from_str(json.as_str()).expect("Error parsing json file");
        
        property
    }

    #[cfg(test)]
    pub fn write_to_json_file(&self, path: &str) -> anyhow::Result<(), anyhow::Error> {
        let json_file = File::create(path)?;

        serde_json::to_writer(json_file, &self)?;

        Ok(())
    }

    pub fn parse_from_str(xml: &str) -> ForestPropertyData {
        quick_xml::de::from_str(xml).expect("Could not parse the XML")
    }

    // Parcels are not probably needed in this context but its good to keep them just in case
    pub fn choose_parcel(&self) -> Parcel {
        let parcels: &Vec<Parcel> = &self.real_estates.real_estate.first().unwrap().parcels.parcel;

        println!("\nParcels:");
        for (i, parcel) in parcels.iter().enumerate() {
            print!("{}. {:?}, ", i, parcel.parcel_number);
        }
        println!("Choose a parcel to view: ");

        let parcel_index = read_number_cli(0, parcels.len());
        parcels[parcel_index].to_owned()
    }

    pub fn get_stand_cli(&self) -> Stand {
        let real_estates = &self.real_estates.real_estate;

        println!("Realestates:");
        for (i, real_estate) in real_estates.iter().enumerate() {
            println!("{}. {:?}, ", i.to_string(), real_estate.real_estate_name);
        }
        println!("Choose a realestate number to view: ");

        let real_estate_index = read_number_cli(0, real_estates.len());

        let real_estate = &self.real_estates.real_estate[real_estate_index];

        let stands_data: Vec<&Stand> = real_estate.get_stands();

        for (i, stand) in stands_data.iter().enumerate() {
            let StandBasicData {
                area, stand_number, ..
            } = &stand.stand_basic_data;

            println!(
                "{}. {:?}, {}ha, stand number: {}",
                i, stand.id, area, stand_number
            )
        }

        println!("Choose a stand:");

        let stand_index = read_number_cli(0, stands_data.len());

        let stand = stands_data[stand_index];

        stand.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RealEstates {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "RealEstate")]
    pub real_estate: Vec<RealEstate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RealEstate {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MunicipalityNumber")]
    pub municipality_number: String,
    #[serde(rename = "AreaNumber")]
    pub area_number: u16,
    #[serde(rename = "GroupNumber")]
    pub group_number: u16,
    #[serde(rename = "UnitNumber")]
    pub unit_number: u16,
    #[serde(rename = "RealEstateName")]
    pub real_estate_name: String,
    #[serde(rename = "Parcels")]
    pub parcels: Parcels,
}

impl RealEstate {
    
    pub fn get_stands(&self) -> Vec<&Stand> {

        let parcels = &self.parcels.parcel;

        let stands_data: Vec<&Stand> = parcels
            .iter()
            .flat_map(|parcel: &Parcel| &parcel.stands.stand)
            .collect();

        stands_data
    } 

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Parcels {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Parcel")]
    pub parcel: Vec<Parcel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Parcel {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ParcelNumber")]
    pub parcel_number: i64,
    #[serde(rename = "Stands")]
    pub stands: Stands,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StandBasicData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Identifiers")]
    pub identifiers: Option<StIdentifiers>,
    #[serde(rename = "CuttingRestriction")]
    pub cutting_restriction: Option<String>,
    #[serde(rename = "StandInfo")]
    pub stand_info: Option<String>,
    #[serde(rename = "DitchingYear")]
    pub ditching_year: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "ChangeTime")]
    pub change_time: String,
    #[serde(rename = "CompleteState")]
    pub complete_state: String,
    #[serde(rename = "StandNumber")]
    pub stand_number: String,
    #[serde(rename = "StandNumberExtension")]
    pub stand_number_extension: String,
    #[serde(rename = "MainGroup")]
    pub main_group: String,
    #[serde(rename = "StandBasicDataDate")]
    pub stand_basic_data_date: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "PolygonGeometry")]
    pub polygon_geometry: PolygonGeometry,
    #[serde(rename = "AreaDecrease")]
    pub area_decrease: Option<String>,
    #[serde(rename = "Accessibility")]
    pub accessibility: Option<String>,
    #[serde(rename = "MainTreeSpecies")]
    pub main_tree_species: Option<String>,
    #[serde(rename = "StandQuality")]
    pub stand_quality: Option<String>,
    #[serde(rename = "DevelopmentClass")]
    pub development_class: Option<String>,
    #[serde(rename = "DrainageState")]
    pub drainage_state: Option<String>,
    #[serde(rename = "SoilType")]
    pub soil_type: Option<String>,
    #[serde(rename = "FertilityClass")]
    pub fertility_class: Option<String>,
    #[serde(rename = "SubGroup")]
    pub sub_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StIdentifiers {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Identifier")]
    pub identifier: StIdentifier,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StIdentifier {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "IdentifierType")]
    pub identifier_type: String,
    #[serde(rename = "IdentifierValue")]
    pub identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpecialFeatures {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SpecialFeature")]
    pub special_feature: Vec<SpecialFeature>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpecialFeature {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "FeatureAdditionalCode")]
    pub feature_additional_code: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "FeatureCode")]
    pub feature_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Operations {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Operation")]
    pub operation: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Operation {
    #[serde(rename = "@mainType")]
    pub main_type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CompletionData")]
    pub completion_data: Option<CompletionData>,
    #[serde(rename = "OperationInfo")]
    pub operation_info: Option<String>,
    #[serde(rename = "Specifications")]
    pub specifications: Option<Specifications>,
    #[serde(rename = "Silviculture")]
    pub silviculture: Option<Silviculture>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "ChangeTime")]
    pub change_time: String,
    #[serde(rename = "OperationType")]
    pub operation_type: String,
    #[serde(rename = "ProposalData")]
    pub proposal_data: ProposalData,
    #[serde(rename = "Cutting")]
    pub cutting: Option<Cutting>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CompletionData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CompletionDate")]
    pub completion_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Specifications {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Specification")]
    pub specification: Vec<Specification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Specification {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "SpecificationCode")]
    pub specification_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Silviculture {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProposalData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ProposalType")]
    pub proposal_type: String,
    #[serde(rename = "ProposalYear")]
    pub proposal_year: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cutting {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CuttingVolume")]
    pub cutting_volume: String,
    #[serde(rename = "Assortments")]
    pub assortments: Option<Assortments>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Assortments {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assortment")]
    pub assortment: Vec<Assortment>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Assortment {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: String,
    #[serde(rename = "StemType")]
    pub stem_type: String,
    #[serde(rename = "AssortmentVolume")]
    pub assortment_volume: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TreeStandData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStandDataDate")]
    pub tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TreeStandDataDate {
    #[serde(rename = "@date", default)]
    pub date: String,
    #[serde(rename = "@type", default)]
    pub tree_stand_data_date_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "DeadTreeStrata")]
    pub dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename = "TreeStrata")]
    pub tree_strata: TreeStrata,
    #[serde(rename = "TreeStandSummary")]
    pub tree_stand_summary: Option<TreeStandSummary>,
}
/*            "TreeStratum": {
  "ChangeState": 0,
  "StratumNumber": 0,
  "TreeSpecies": 1,
  "Storey": 1,
  "Age": 90,
  "BasalArea": 2,
  "MeanDiameter": 31,
  "MeanHeight": 23,
  "DataSource": 0
} */
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeadTreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "DeadTreeStratum")]
    pub dead_tree_stratum: Vec<DeadTreeStratum>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeadTreeStratum {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: i32,
    #[serde(rename = "DeadTreeType")]
    pub dead_tree_type: i32,
    #[serde(rename = "TreeSpecies")]
    pub tree_species: i32,
    #[serde(rename = "Volume")]
    pub volume: Option<f64>,
    #[serde(rename = "MeanDiameter")]
    pub mean_diameter: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStratum")]
    pub tree_stratum: Vec<TreeStratum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeStratum {
    #[serde(rename = "@id", default)]
    pub id: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TreeStandSummary {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PulpWoodVolume")]
    pub pulp_wood_volume: Option<String>,
    #[serde(rename = "SawLogVolume")]
    pub saw_log_volume: Option<String>,
    #[serde(rename = "ChangeState")]
    pub change_state: String,
    #[serde(rename = "MeanAge")]
    pub mean_age: f64,
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
    #[serde(rename = "ValueGrowthPercent")]
    pub value_growth_percent: Option<f64>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}
