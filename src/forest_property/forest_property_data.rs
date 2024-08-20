use std::{fs::File, io::Read, sync::Arc};

use anyhow::Ok;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{Deserializer, EventReader, ParserConfig};

use crate::forest_property::stand;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ForestPropertyData {
    #[serde(default)]
    pub xmlns: String,
    #[serde(rename(deserialize = "xmlns:re"), default)]
    pub xmlns_re: String,
    #[serde(rename(deserialize = "xmlns:st"), default)]
    pub xmlns_st: String,
    #[serde(rename(deserialize = "xmlns:ts"), default)]
    pub xmlns_ts: String,
    #[serde(rename(deserialize = "xmlns:tst"), default)]
    pub xmlns_tst: String,
    #[serde(rename(deserialize = "xmlns:dts"), default)]
    pub xmlns_dts: String,
    #[serde(rename(deserialize = "xmlns:tss"), default)]
    pub xmlns_tss: String,
    #[serde(rename(deserialize = "xmlns:op"), default)]
    pub xmlns_op: String,
    #[serde(rename(deserialize = "xmlns:sf"), default)]
    pub xmlns_sf: String,
    #[serde(rename(deserialize = "xmlns:gdt"), default)]
    pub xmlns_gdt: String,
    #[serde(rename(deserialize = "xmlns:co"), default)]
    pub xmlns_co: String,
    #[serde(rename(deserialize = "xmlns:gml"), default)]
    pub xmlns_gml: String,
    #[serde(rename(deserialize = "xmlns:xsi"), default)]
    pub xmlns_xsi: String,
    #[serde(rename(deserialize = "xmlns:xlink"), default)]
    pub xmlns_xlink: String,
    #[serde(rename(deserialize = "schemaLocation"), default)]
    pub xsi_schema_location: String,
    #[serde(rename(deserialize = "$text"), default)]
    pub text: Option<String>,
    #[serde(rename(deserialize = "RealEstates"), default)]
    pub real_estates: Vec<RealEstates>,
}

impl ForestPropertyData {
    pub fn from_xml_file(path: &str) -> ForestPropertyData {
        let xml_file = File::open(path).expect("error opening the xml file");
        let config = ParserConfig::new()
            .trim_whitespace(false)
            .whitespace_to_characters(true);

        let event_reader = EventReader::new_with_config(xml_file, config);

        let deserializer = &mut Deserializer::new(event_reader);

        let property = ForestPropertyData::deserialize(deserializer)
            .expect("Unable to deserialize the xml file");

        property
    }

    pub fn write_to_json_file(&self, path: &str) -> anyhow::Result<()> {
        let json_file = File::create(path)?;

        serde_json::to_writer(json_file, &self)?;

        Ok(())
    }

    pub fn choose_real_estate(&self) -> &RealEstate {
        let real_estates: &Vec<RealEstates> = &self.real_estates;
        let mut real_estate_number = String::new();

        println!("\nRealestates:");
        for (i, real_estate_parent) in real_estates.iter().enumerate() {
            print!(
                "{}. {:?}, ",
                i.to_string(),
                real_estate_parent.real_estate.real_estate_name
            );
        }

        println!("Choose a realestate number to view: ");

        // Read parcel number from user input into String `parcel_number`
        std::io::stdin()
            .read_line(&mut real_estate_number)
            .expect("Failed to read line");

        // Shadowing `parcel_number` to convert it to an integer
        let index: usize = real_estate_number
            .trim()
            .parse()
            .expect("Please type a number!");

        &self.real_estates[index].real_estate
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct RealEstates {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "RealEstate"))]
    pub real_estate: RealEstate,
}

impl RealEstate {
    pub fn choose_parcel(&self) -> Parcel {
        let parcels: &Vec<Parcel> = &self
            .parcels.iter().flat_map(|f| {
                f.parcel.clone()
            }).collect();
        let mut parcel_number = String::new();

        println!("\nParcels:");
        for parcel in parcels.iter() {
            print!("{:?}, ", parcel.parcel_number);
        }

        println!("Choose a parcel number to view: ");

        // Read parcel number from user input into String `parcel_number`
        std::io::stdin()
            .read_line(&mut parcel_number)
            .expect("Failed to read line");

        // Shadowing `parcel_number` to convert it to an integer
        let parcel_number: i64 = parcel_number.trim().parse().expect("Please type a number!");
        let parcel = parcels
            .iter()
            .find(|&x| x.parcel_number == parcel_number)
            .unwrap();

        parcel.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct RealEstate {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "MunicipalityNumber"))]
    pub municipality_number: String,
    #[serde(rename(deserialize = "AreaNumber"))]
    pub area_number: String,
    #[serde(rename(deserialize = "GroupNumber"))]
    pub group_number: String,
    #[serde(rename(deserialize = "UnitNumber"))]
    pub unit_number: String,
    #[serde(rename(deserialize = "RealEstateName"))]
    pub real_estate_name: String,
    #[serde(rename(deserialize = "Parcels"))]
    pub parcels: Vec<Parcels>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Parcels {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Parcel"))]
    pub parcel: Vec<Parcel>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Parcel {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ParcelNumber"))]
    pub parcel_number: i64,
    #[serde(rename(deserialize = "Stands"))]
    pub stands: Option<Stands>,
}

impl Parcel {
    pub fn choose_stand(&self) -> Stand {
        let mut stand_number = String::new();

        let stands: Vec<Stand> = self.stands.iter().flat_map(|f| f.stand.clone()).collect();

        println!("\nStands:");

            for (i, stand) in stands.iter().enumerate() {
   
                if stand.tree_stand_data.is_some() {
                    print!("\n{}. {:?}, area: {}ha", i, stand.stand_basic_data.stand_number, stand.stand_basic_data.area);
                }
            }
        

        println!("Choose a stand number to view: ");

        std::io::stdin().read_line(&mut stand_number).expect("Failed to read line");

        let stand_number: usize = stand_number.trim().parse().expect("Please type a number!");

        stands[stand_number].clone()

    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Stands {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Stand"))]
    pub stand: Vec<Stand>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Stand {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "StandBasicData"))]
    pub stand_basic_data: StandBasicData,
    #[serde(rename(deserialize = "SpecialFeatures"), default)]
    pub special_features: Vec<SpecialFeatures>,
    #[serde(rename(deserialize = "Operations"), default)]
    pub operations: Option<Operations>,
    #[serde(rename(deserialize = "TreeStandData"), default)]
    pub tree_stand_data: Option<TreeStandData>,
}

impl Stand {
    pub fn get_coordinate_string(&self) -> Vec<String>{

        let gml_exterior = self.stand_basic_data.gpolygon_geometry.gml_polygon_property.gml_polygon.exterior.to_owned();
        let gml_interior = self.stand_basic_data.gpolygon_geometry.gml_polygon_property.gml_polygon.interior.to_owned();

        let gml: Vec<Gml> = [gml_exterior, gml_interior].concat();

        let coordinates: Vec<String> = gml.to_owned().into_iter().map(|f| f.linear_ring.coordinates).collect();

        coordinates
    }

      // Get stem count
      pub fn get_stem_count(&self) -> i64 {
        let data_date = self.tree_stand_data.as_ref().unwrap().tree_stand_data_date.last().unwrap();
        let stem_count = data_date.tree_stand_summary.as_ref().unwrap().stem_count;
        
        stem_count
    }

    // Determines if stem count is in individual stratum
    pub fn stem_count_in_stratum(&self) -> bool {
        if let Some(tree_stand_data) = &self.tree_stand_data {
            let data_date = tree_stand_data.tree_stand_data_date.last().unwrap();
            for stratum in data_date.tree_strata.tree_stratum.iter() {
                if stratum.stem_count.is_some() {
                    return true;
                }
            }
        }

        false
    }

    // Returns strata information for the stand
    pub fn get_stratums(&self) -> Vec<TreeStratum> {
        let tree_stand_data = self.tree_stand_data.as_ref().unwrap();
        let data_date = tree_stand_data.tree_stand_data_date.last().unwrap();

        data_date.tree_strata.tree_stratum.to_owned()

    }
}



#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct StandBasicData {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Identifiers"), default)]
    pub identifiers: Vec<StIdentifiers>,
    #[serde(rename(deserialize = "CuttingRestriction"))]
    pub cutting_restriction: Option<i32>,
    #[serde(rename(deserialize = "StandInfo"))]
    pub stand_info: Option<String>,
    #[serde(rename(deserialize = "DitchingYear"))]
    pub ditching_year: Option<i32>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: String,
    #[serde(rename(deserialize = "ChangeTime"))]
    pub change_time: String,
    #[serde(rename(deserialize = "CompleteState"))]
    pub complete_state: String,
    #[serde(rename(deserialize = "StandNumber"))]
    pub stand_number: i64,
    #[serde(rename(deserialize = "StandNumberExtension"))]
    pub stand_number_extension: String,
    #[serde(rename(deserialize = "MainGroup"))]
    pub main_group: String,
    #[serde(rename(deserialize = "StandBasicDataDate"))]
    pub stand_basic_data_date: String,
    #[serde(rename(deserialize = "Area"))]
    pub area: String,
    #[serde(rename(deserialize = "PolygonGeometry"))]
    pub gpolygon_geometry: GdtPolygonGeometry,
    #[serde(rename(deserialize = "AreaDecrease"))]
    pub area_decrease: Option<String>,
    #[serde(rename(deserialize = "Accessibility"))]
    pub accessibility: Option<String>,
    #[serde(rename(deserialize = "MainTreeSpecies"))]
    pub main_tree_species: Option<String>,
    #[serde(rename(deserialize = "StandQuality"))]
    pub stand_quality: Option<String>,
    #[serde(rename(deserialize = "DevelopmentClass"))]
    pub development_class: Option<String>,
    #[serde(rename(deserialize = "DrainageState"))]
    pub drainage_state: Option<String>,
    #[serde(rename(deserialize = "SoilType"))]
    pub soil_type: Option<String>,
    #[serde(rename(deserialize = "FertilityClass"))]
    pub fertility_class: Option<String>,
    #[serde(rename(deserialize = "SubGroup"))]
    pub sub_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct StIdentifiers {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Identifier"))]
    pub st_identifier: StIdentifier,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct StIdentifier {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "IdentifierType"))]
    pub co_identifier_type: String,
    #[serde(rename(deserialize = "IdentifierValue"))]
    pub co_identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GdtPolygonGeometry {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "pointProperty"))]
    pub gml_point_property: GmlPointProperty,
    #[serde(rename(deserialize = "polygonProperty"))]
    pub gml_polygon_property: GmlPolygonProperty,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GmlPointProperty {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Point"))]
    pub gml_point: GmlPoint,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GmlPoint {
    #[serde(rename(deserialize = "srsName"))]
    pub srs_name: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "coordinates"))]
    pub gml_coordinates: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GmlPolygonProperty {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Polygon"))]
    pub gml_polygon: GmlPolygon,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GmlPolygon {
    #[serde(rename(deserialize = "srsName"))]
    pub srs_name: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "interior"), default)]
    pub interior: Vec<Gml>,
    #[serde(rename(deserialize = "exterior"), default)]
    pub exterior: Vec<Gml>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Gml {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "LinearRing"))]
    pub linear_ring: GmlLinearRing,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct GmlLinearRing {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "coordinates"))]
    pub coordinates: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct SpecialFeatures {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "SpecialFeature"))]
    pub special_feature: Vec<SpecialFeature>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct SpecialFeature {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "FeatureAdditionalCode"))]
    pub feature_additional_code: Option<Vec<i32>>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "FeatureCode"))]
    pub feature_code: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Operations {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Operation"), default)]
    pub operation: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Operation {
    #[serde(rename(deserialize = "mainType"))]
    pub main_type: String,
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "CompletionData"))]
    pub completion_data: Option<Vec<CompletionData>>,
    #[serde(rename(deserialize = "OperationInfo"))]
    pub operation_info: Option<String>,
    #[serde(rename(deserialize = "Specifications"))]
    pub specifications: Option<Specifications>,
    #[serde(rename(deserialize = "Silviculture"))]
    pub silviculture: Option<OpSilviculture>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: String,
    #[serde(rename(deserialize = "ChangeTime"))]
    pub change_time: String,
    #[serde(rename(deserialize = "OperationType"))]
    pub operation_type: i32,
    #[serde(rename(deserialize = "ProposalData"))]
    pub proposal_data: OpProposalData,
    #[serde(rename(deserialize = "Cutting"))]
    pub cutting: Option<Cutting>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct CompletionData {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "CompletionDate"))]
    pub completion_date: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Specifications {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Specification"))]
    pub specification: Vec<Specification>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Specification {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "SpecificationCode"))]
    pub specification_code: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct OpSilviculture {}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct OpProposalData {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ProposalType"))]
    pub proposal_type: i32,
    #[serde(rename(deserialize = "ProposalYear"))]
    pub proposal_year: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Cutting {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "CuttingVolume"))]
    pub cutting_volume: f64,
    #[serde(rename(deserialize = "Assortments"), default)]
    pub assortments: Option<Assortments>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Assortments {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "Assortment"))]
    pub assortment: Vec<Assortment>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Assortment {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "TreeSpecies"))]
    pub tree_species: Option<String>,
    #[serde(rename(deserialize = "StemType"))]
    pub stem_type: i32,
    #[serde(rename(deserialize = "AssortmentVolume"))]
    pub assortment_volume: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TreeStandData {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "TreeStandDataDate"))]
    pub tree_stand_data_date: Vec<TreeStandDataDate>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TreeStandDataDate {
    pub date: String,
    #[serde(rename(deserialize = "type"))]
    pub ts_tree_stand_data_date_type: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "DeadTreeStrata"))]
    pub dead_tree_strata: Option<DeadTreeStrata>,
    #[serde(rename(deserialize = "TreeStrata"))]
    pub tree_strata: TreeStrata,
    #[serde(rename(deserialize = "TreeStandSummary"))]
    pub tree_stand_summary: Option<TreeStandSummary>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct DeadTreeStrata {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "DeadTreeStratum"))]
    pub dead_tree_stratum: Vec<DeadTreeStratum>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct DeadTreeStratum {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "DeadTreeType"))]
    pub dead_tree_type: i32,
    #[serde(rename(deserialize = "TreeSpecies"))]
    pub tree_species: i32,
    #[serde(rename(deserialize = "Volume"))]
    pub volume: Option<f64>,
    #[serde(rename(deserialize = "MeanDiameter"))]
    pub mean_diameter: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TreeStrata {
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "TreeStratum"))]
    pub tree_stratum: Vec<TreeStratum>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TreeStratum {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "StratumNumber"))]
    pub stratum_number: i32,
    #[serde(rename(deserialize = "TreeSpecies"))]
    pub tree_species: i32,
    #[serde(rename(deserialize = "Storey"))]
    pub storey: i32,
    #[serde(rename(deserialize = "Age"))]
    pub age: i32,
    #[serde(rename(deserialize = "MeanHeight"))]
    pub mean_height: f64,
    #[serde(rename(deserialize = "DataSource"))]
    pub data_source: i32,
    #[serde(rename(deserialize = "VolumeGrowth"))]
    pub volume_growth: Option<f64>,
    #[serde(rename(deserialize = "PulpWoodVolume"))]
    pub pulp_wood_volume: Option<f64>,
    #[serde(rename(deserialize = "SawLogVolume"))]
    pub saw_log_volume: Option<f64>,
    #[serde(rename(deserialize = "SawLogPercent"))]
    pub saw_log_percent: Option<f64>,
    #[serde(rename(deserialize = "Volume"))]
    pub volume: Option<f64>,
    #[serde(rename(deserialize = "MeanDiameter"))]
    pub mean_diameter: Option<f64>,
    #[serde(rename(deserialize = "StemCount"))]
    pub stem_count: Option<i32>,
    #[serde(rename(deserialize = "BasalArea"))]
    pub basal_area: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TreeStandSummary {
    pub id: String,
    #[serde(rename(deserialize = "$text"))]
    pub text: Option<String>,
    #[serde(rename(deserialize = "PulpWoodVolume"))]
    pub pulp_wood_volume: Option<f64>,
    #[serde(rename(deserialize = "SawLogVolume"))]
    pub saw_log_volume: Option<f64>,
    #[serde(rename(deserialize = "ChangeState"))]
    pub change_state: i32,
    #[serde(rename(deserialize = "MeanAge"))]
    pub mean_age: f64,
    #[serde(rename(deserialize = "BasalArea"))]
    pub basal_area: f64,
    #[serde(rename(deserialize = "StemCount"))]
    pub stem_count: i64,
    #[serde(rename(deserialize = "MeanDiameter"))]
    pub mean_diameter: f64,
    #[serde(rename(deserialize = "MeanHeight"))]
    pub mean_height: f64,
    #[serde(rename(deserialize = "Volume"))]
    pub volume: f64,
    #[serde(rename(deserialize = "VolumeGrowth"))]
    pub volume_growth: f64,
    #[serde(rename(deserialize = "ValueGrowthPercent"))]
    pub value_growth_percent: Option<f64>,
    #[serde(rename(deserialize = "Value"))]
    pub value: Option<f64>,
}
