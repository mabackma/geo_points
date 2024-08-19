use crate::forest_property::stand::{Stands, Stand};
use serde::{Deserialize, Serialize};

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

impl Parcel {
    // Choose a stand
    pub fn choose_stand(&self) -> Stand {
        let mut stand_number = String::new();

        println!("\nStands:");
        for stand in self.stands.stand.iter() {
            if stand.tree_stand_data.is_some() {
                print!("{:?}, ", stand.stand_basic_data.stand_number);
            }
        }

        println!("Choose a stand number to view: ");

        // Read stand number from user input into String `stand_number`
        std::io::stdin().read_line(&mut stand_number).expect("Failed to read line");

        // Shadowing `stand_number` to convert it to an integer
        let stand_number: i64 = stand_number.trim().parse().expect("Please type a number!");
        let stand = self.stands.stand.iter().find(|&x| x.stand_basic_data.stand_number == stand_number).unwrap();

        stand.clone()
    }
}