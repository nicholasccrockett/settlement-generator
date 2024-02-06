use crate::sst::Parameters;
use crate::sst::Settlement;
use crate::sst::generate_settlement;
use serde::{Serialize,Deserialize};
use serde_json::{Result,Value,json};
// use std::io::prelude::*;
// use std::fs::File; // uncomment if implementing save functionality

/* To Do */
// [X] - Implement all of the Parameters (currently only name)
// [X] - Implement Test scenarios
// [ ] - Handle improper parameter options
// [ ] - Implement read and write to save file
// [ ] - Implement interactive response
// [ ] - Implement Town Generation
// [ ] - Implement City Generation
// [ ] - Implement Capital Generation
// [ ] - Implement Fortress Generation


pub mod sst;

fn main() -> std::io::Result<()>{
    // let mut p = Parameters{ name: None, origin: None, specialty: None, age: None, condition: None, size: None, environment: None, resident_population: None, demographics: None, disposition: None, law_enforcement: None, leadership: None, population_wealth: None, number_shops: None, number_services: None, rare_magic: None};
    
    // p.name = Some(String::from("Test Build"));
    // p.origin = Some(String::from("Accidental"));
    // p.specialty = Some(String::from("Hospitality"));
    // p.age = Some(String::from("Ancient"));
    // p.environment = Some(String::from("Tundra"));
    // p.condition = Some(String::from("Ramshackle"));
    // p.disposition = Some(String::from("Neutral"));
    // p.demographics = Some(String::from("One"));
    // p.size = Some(String::from("Large"));

    let p = crate::sst::Parameters{ name: Some(String::from("Correct Parameter")), 
        origin: Some(String::from("Native")), 
        specialty: Some(String::from("Hospitality")), 
        age: Some(String::from("Recent")), 
        condition: Some(String::from("Fair")), 
        size: Some(String::from("Medium")), 
        environment: Some(String::from("Forest")), 
        resident_population: Some(String::from("Appropriate")), 
        demographics: Some(String::from("Normal")), 
        disposition: Some(String::from("Neutral")), 
        law_enforcement: Some(String::from("SmallLocalWatch")), 
        leadership: Some(String::from("Hereditary")), 
        population_wealth: Some(String::from("Average")), 
        number_shops: Some(1), 
        number_services: Some(1), 
        rare_magic: Some(false)
    };

    let x: Settlement = generate_settlement(p);

    let data = serde_json::ser::to_string_pretty(&x).unwrap();

    // to save data
    // let mut file = File::create("test.json")?;
    // file.write(data.as_bytes())?;

    println!("{}", data);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn no_parameters() {
        let p = crate::sst::Parameters{ name: None, origin: None, specialty: None, age: None, condition: None, size: None, environment: None, resident_population: None, demographics: None, disposition: None, law_enforcement: None, leadership: None, population_wealth: None, number_shops: None, number_services: None, rare_magic: None};
        let x: crate::sst::Settlement = crate::sst::generate_settlement(p);
    
        let data = serde_json::ser::to_string_pretty(&x).unwrap();
        
        
    println!("{}", data);println!("{}", data);
    }

    #[test]
    fn correct_parameters() {
        let p = crate::sst::Parameters{ name: Some(String::from("Correct Parameter")), 
            origin: Some(String::from("Native")), 
            specialty: Some(String::from("Hospitality")), 
            age: Some(String::from("Recent")), 
            condition: Some(String::from("Fair")), 
            size: Some(String::from("Medium")), 
            environment: Some(String::from("Forest")), 
            resident_population: Some(String::from("Appropriate")), 
            demographics: Some(String::from("Normal")), 
            disposition: Some(String::from("Neutral")), 
            law_enforcement: Some(String::from("SmallLocalWatch")), 
            leadership: Some(String::from("Hereditary")), 
            population_wealth: Some(String::from("Average")), 
            number_shops: Some(1), 
            number_services: Some(1), 
            rare_magic: Some(false)
        };
        let x = crate::sst::generate_settlement(p);
    
        let data = serde_json::ser::to_string_pretty(&x).unwrap();
        
        
    println!("{}", data);println!("{}", data);
    }    

    #[test]
    fn incorrect_parameters() {
        let p = crate::sst::Parameters{ name: Some(String::from("Incorrect Parameter")), 
            origin: Some(String::from("Wrong")), 
            specialty: Some(String::from("Wrong")), 
            age: Some(String::from("Wrong")), 
            condition: Some(String::from("Wrong")), 
            size: Some(String::from("Wrong")), 
            environment: Some(String::from("Wrong")), 
            resident_population: Some(String::from("Wrong")), 
            demographics: Some(String::from("Wrong")), 
            disposition: Some(String::from("Wrong")), 
            law_enforcement: Some(String::from("Wrong")), 
            leadership: Some(String::from("Wrong")), 
            population_wealth: Some(String::from("Wrong")), 
            number_shops: Some(0), 
            number_services: Some(0), 
            rare_magic: Some(false)
        };
        let x: crate::sst::Settlement = crate::sst::generate_settlement(p);
    
        let data = serde_json::ser::to_string_pretty(&x).unwrap();
        
        println!("{}", data);println!("{}", data);
    }    
}