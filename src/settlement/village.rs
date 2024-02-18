use crate::settlement::*;

pub struct Village {
    name: String,
    origin: Origin,
    specialty: Specialty,
    age: Age,
    condition: Condition,
    visitor_traffic: VisitorTraffic,
    size: Size,
    environment: Environment,

    resident_population: ResidentPopulation,
    demographics: Demographics,
    disposition: Disposition,
    law_enforcement: LawEnforcement,
    leadership: Leadership,
    population_wealth: PopulationWealth,
    crime: Crime,

    shops: Vec<Shop>,
    services: Vec<Service>,
    places_of_worship: Vec<PlaceOfWorship>
}
