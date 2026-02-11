use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CityEntry {
    pub city: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct VehicleBrand {
    pub brand: String,
    pub models: Vec<String>,
}

pub struct Datasets;

impl Datasets {
    pub fn get_names() -> Vec<&'static str> {
        serde_json::from_str(include_str!("data/names.json")).unwrap()
    }

    pub fn get_surnames() -> Vec<&'static str> {
        serde_json::from_str(include_str!("data/surnames.json")).unwrap()
    }

    pub fn get_cities() -> Vec<CityEntry> {
        serde_json::from_str(include_str!("data/cities.json")).unwrap()
    }

    pub fn get_email_domains() -> Vec<&'static str> {
        serde_json::from_str(include_str!("data/emails.json")).unwrap()
    }

    pub fn get_vehicles() -> Vec<VehicleBrand> {
        serde_json::from_str(include_str!("data/vehicles.json")).unwrap()
    }

    pub fn random_name() -> String {
        let names = Self::get_names();
        let surnames = Self::get_surnames();
        let mut rng = rand::thread_rng();

        let first = names.choose(&mut rng).unwrap();
        let last1 = surnames.choose(&mut rng).unwrap();
        let last2 = surnames.choose(&mut rng).unwrap();

        format!("{} {} {}", first, last1, last2)
    }
}
