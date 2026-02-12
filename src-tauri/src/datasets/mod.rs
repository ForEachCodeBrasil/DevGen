use rand::seq::SliceRandom;
use rand::Rng;
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

#[derive(Deserialize)]
pub struct AddressData {
    pub street_types: Vec<String>,
    pub street_names: Vec<String>,
    pub neighborhoods: Vec<String>,
}

#[derive(Deserialize)]
pub struct NickData {
    pub adjectives: Vec<String>,
    pub nouns: Vec<String>,
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

    pub fn get_addresses() -> AddressData {
        serde_json::from_str(include_str!("data/addresses.json")).unwrap()
    }

    pub fn get_nicks() -> NickData {
        serde_json::from_str(include_str!("data/nicks.json")).unwrap()
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

    pub fn random_address() -> String {
        let addr = Self::get_addresses();
        let mut rng = rand::thread_rng();

        let street_type = addr.street_types.choose(&mut rng).unwrap();
        let street_name = addr.street_names.choose(&mut rng).unwrap();
        let neighborhood = addr.neighborhoods.choose(&mut rng).unwrap();
        let number = rng.gen_range(1..2000);

        format!(
            "{}, {} - {}, {}",
            street_type, street_name, number, neighborhood
        )
    }

    pub fn random_cep() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{:05}-{:03}",
            rng.gen_range(1000..99999),
            rng.gen_range(0..999)
        )
    }

    pub fn random_nick() -> String {
        let nicks = Self::get_nicks();
        let mut rng = rand::thread_rng();

        let adj = nicks.adjectives.choose(&mut rng).unwrap();
        let noun = nicks.nouns.choose(&mut rng).unwrap();

        match rng.gen_range(0..4) {
            0 => format!("{}{}", adj, noun),
            1 => format!("{}_{}", adj, noun),
            2 => format!("{}{}{}", adj, noun, rng.gen_range(10..999)),
            _ => format!("{}.{}", adj.to_lowercase(), noun.to_lowercase()),
        }
    }
}
