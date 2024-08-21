#[allow(dead_code)]

mod state;
use state::{Driver, Bundle, State};

fn main() {
    // create a basic state with 1 driver and bundle each
    let driver = Driver{id: "driver-123".to_string()};
    let bundle = Bundle{id: "bundle-456".to_string()};
    let drivers = vec![driver];
    let bundles = vec![bundle];
    let state = State::new(drivers, bundles);

    // print some info about the state
    println!("State contains the following drivers:");
    for driver in state.drivers.values() {
        println!("{}", (*driver).id);
    }
    println!("State contains the following bundles:");
    for bundle in state.bundles.values() {
        println!("{}", (*bundle).id);
    }
}
