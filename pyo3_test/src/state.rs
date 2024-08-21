use std::collections::HashMap;
use std::vec::Vec;

pub struct Driver {
    pub id: String,
}

pub struct Bundle {
    pub id: String,
}

pub struct EligiblityRelationship{
    pub driver: Driver,
    pub bundle: Bundle,
}

pub struct State {
    pub drivers: HashMap<String, Driver>,
    pub bundles: HashMap<String, Bundle>,
}

impl State {
    pub fn new(drivers: Vec<Driver>, bundles: Vec<Bundle>) -> State {
        let mut driver_map: HashMap<String, Driver> = HashMap::new();
        let mut bundle_map: HashMap<String, Bundle> = HashMap::new();
        for driver in drivers {
            driver_map.insert(driver.id.clone(), driver);
        }
        for bundle in bundles {
            bundle_map.insert(bundle.id.clone(), bundle);
        }
        return State{drivers: driver_map, bundles: bundle_map}
    }
}