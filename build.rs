use std::{error::Error};

use asset_generator;

fn main() -> Result<(), Box<dyn Error>> {
    asset_generator::generate_database()
}
