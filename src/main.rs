use crate::sst::Parameters;
use crate::sst::Settlement;
use crate::sst::generate_settlement;
use serde::{Serialize,Deserialize};
use serde_json::{Result,Value,json};
// use std::io::prelude::*;
// use std::fs::File; // uncomment if implementing save functionality

/* To Do */
// [ ] - Implement all of the Parameters (currently only name)
// [ ] - Implement read and write to save file
// [ ] - Implement interactive response
// [ ] - Implement Town Generation
// [ ] - Implement City Generation
// [ ] - Implement Capital Generation
// [ ] - Implement Fortress Generation


pub mod sst;

fn main() -> std::io::Result<()>{
    let mut p = Parameters{ name: None, origin: None, specialty: None, age: None, condition: None, size: None, environment: None, resident_population: None, number_shops: None, number_services: None, rare_magic: None};
    
    p.name = Some(String::from("Test Build"));

    let x: Settlement = generate_settlement(p);

    let data = serde_json::ser::to_string_pretty(&x).unwrap();

    // to save data
    // let mut file = File::create("test.json")?;
    // file.write(data.as_bytes())?;

    println!("{}", data);
    Ok(())
}
