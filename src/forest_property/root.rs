use crate::forest_property::parcel::{Parcels, Parcel};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "ForestPropertyData")]
    pub forest_property_data: ForestPropertyData,
}

impl Root {
    // Choose a parcel 
    pub fn choose_parcel(&self) -> Parcel {
        let parcels: &Vec<Parcel> = &self.forest_property_data.real_estates.real_estate.parcels.parcel;
        let mut parcel_number = String::new();
        
        println!("\nParcels:");
        for parcel in parcels.iter() {
            print!("{:?}, ", parcel.parcel_number);
        }

        println!("Choose a parcel number to view: ");

        // Read parcel number from user input into String `parcel_number`
        std::io::stdin().read_line(&mut parcel_number).expect("Failed to read line");

        // Shadowing `parcel_number` to convert it to an integer
        let parcel_number: i64 = parcel_number.trim().parse().expect("Please type a number!");
        let parcel = parcels.iter().find(|&x| x.parcel_number == parcel_number).unwrap();

        parcel.clone()
    }
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