use rand::Rng;
use serde::{Deserialize, Serialize};
// use serde_json::Result;

//Step 1: Basic Information
#[derive(Debug,Serialize,Deserialize)]
enum Origin {
    Accidental,
    BusinessVenture,
    Crossroads,
    MilitaryOutpost,
    NoMansLand,
    Native,
    OvernightStop,
    WildernessExpert
}
#[derive(Debug,Serialize,Deserialize)]
enum Age {
    Recent,
    Established,
    Mature,
    Old,
    Ancient,
    Unknown
}
#[derive(Debug,Serialize,Deserialize)]
enum Specialty {
    AtypicalShippingMethods,
    ExcellentAndUniqueFood,
    PlentifulAndVariedHighQualityBeverages,
    Hospitality,
    Information,
    PurchasingConnections,
    UnscrupulousContractors
}
#[derive(Debug,Serialize,Deserialize)]
enum Condition {
    Ramshackle,
    Poor,
    Fair,
    Good,
    Immaculate
}
#[derive(Debug,Serialize,Deserialize)]
enum VisitorTraffic {
    Vacant,
    Groups,
    Crowds,
    Droves,
    Masses
}
#[derive(Debug,Serialize,Deserialize)]
enum Size {
    VerySmall,
    Small,
    Medium,
    Large,
    VeryLarge
}
#[derive(Debug,Serialize,Deserialize)]
enum Environment {
    Coastal,
    Forest,
    Mountains,
    Plains,
    River,
    Swamp,
    Underground,
    Valley,
    Tundra,
    Desert
}
//Step 2:Community
#[derive(Debug,Serialize,Deserialize)]
enum ResidentPopulation {
    NearlyDeserted,
    Sparse,
    Appropriate,
    Congested,
    Overwhelmed
}
#[derive(Debug,Serialize,Deserialize)]
enum Demographics {
    One,
    Two,
    Normal,
    Wide,
    HighAndLow,
    EverChanging
}
#[derive(Debug,Serialize,Deserialize)]
enum Disposition {
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Open
}
#[derive(Debug,Serialize,Deserialize)]
enum LawEnforcement {
    None,
    Sheriff,
    SmallLocalWatch,
    WellEquipped,
    OverwhelmingPresence
}
#[derive(Debug,Serialize,Deserialize)]
enum Leadership {
    NoLeader,
    Hereditary,
    MerchantMonarch,
    UnderworldCriminal,
    Plutocracy,
    Magocracy,
    Theocracy,
    Oligarchy,
    LocalCouncil,
    SingleElected,
    AnarchoSyndicalistCommune
}
#[derive(Debug,Serialize,Deserialize)]
enum PopulationWealth {
    Destitute,
    Impoverished,
    Average,
    Prosperous,
    Wealthy,
    Affluent
}
#[derive(Debug,Serialize,Deserialize)]
enum Crime {
    Regular,
    Common,
    Average,
    Uncommon,
    Rare
}
//Step 3: Points of Interest
#[derive(Debug,Serialize,Deserialize)]
enum ShopType {
    Baker,
    Butcher,
    Cooper,
    Carpenter,
    GeneralStore,
    Herbalist,
    Smithy,
    Tailor,
    TannerTaxidermist,
    Thatcher,
    Wainwrite,
    Weaver,
    Alchemist,
    Artist,
    BankAndExchange,
    Cobbler,
    FoundrySmelting,
    Mill,
    Textile,
    Shipwrite,
    RareBotanicals,
    LuxuryFurnishing,
    RareLibationsAndFare,
    RareTradeGoods,
    MagicShopArmor,
    MagicShopBooks,
    MagicShopClothing,
    MagicShopJewelry,
    MagicShopWeapons,
    MagicShopMisellaneous
}
#[derive(Debug,Serialize,Deserialize)]
struct Shop {
    name: String,
    shop_type: ShopType
}

#[derive(Debug,Serialize,Deserialize)]
enum ServiceType {
    Barber,
    Bathhouse,
    Doctor,
    HouseOfLeisure,
    Inn,
    Club,
    Soothsayer,
    Stable,
    Tavern,
    BrutesAndBrawlers,
    CloakAndDagger,
    BowsAndSLings,
    ScribesAndClerks,
    GuidesAndTrackers,
    CaravanAndMount,
    ArcaneAcademics,
    MagicMercenaries,
    PriestlyGuidance,
    HandsOfTheDivine
}

#[derive(Debug,Serialize,Deserialize)]
struct Service {
    name: String,
    service_type: ServiceType,
    hired_help_size: HiredHelpSize
}

#[derive(Debug,Serialize,Deserialize)]
enum HiredHelpSize {
    Individual,
    Team,
    Guild,
    None
}
#[derive(Debug,Serialize,Deserialize)]
enum Quality {
    Poor,
    Good,
    Fine
}

#[derive(Debug,Serialize,Deserialize)]
enum Alignment {
    Evil,
    Neutral,
    Good
}
#[derive(Debug,Serialize,Deserialize)]
enum PlaceOfWorshipSize {
    Secret,
    Altar,
    Oratory,
    Sanctuary,
    Temple,
    GreateTemple
}
#[derive(Debug,Serialize,Deserialize)]
enum Fervency {
    Unseen,
    Quiet,
    Subtle,
    Moderate,
    Fervent,
    Zealous
}
#[derive(Debug,Serialize,Deserialize)]
struct PlaceOfWorship {
    size: PlaceOfWorshipSize,
    fervency: Fervency,
    alignment: Alignment
}

//Part 4: Extra Intrigue
#[derive(Debug,Serialize,Deserialize)]
enum RecentHistory {
    BenevolenceOfYesteryear,
    Construction,
    CrimeCentral,
    Neutrality,
    PopularProducer,
    Wartorn
}
#[derive(Debug,Serialize,Deserialize)]
enum Politics {
    BrinkOfWar,
    LawlessRegion,
    Embattlement,
    Revolution,
    Peace,
    New
}
#[derive(Debug,Serialize,Deserialize)]
enum Event {
    HighClassVisitor,
    Troublemakers,
    FoolsFestival,
    CaughtRedHanded,
    Strangers,
    PublicEmergency
}
#[derive(Debug,Serialize,Deserialize)]
enum Opportunities {
    PoliticalIntrigue,
    MissingPerson,
    Monster,
    GuardDuty,
    Transportation,
    Acquisition
}
#[derive(Debug,Serialize,Deserialize)]
enum Weather {
    Good,
    Mild,
    Fair,
    Harsh,
    Bad
}
#[derive(Debug,Serialize,Deserialize)]
enum DangerLevel {
    EverPresent,
    Frequent,
    Common,
    Uncommon,
    Rare
}
#[derive(Debug,Serialize,Deserialize)]
enum DangerType {
    SuspiciousLocals,
    Raids,
    Monster,
    Environmental,
    Cult
}


#[derive(Debug,Serialize,Deserialize)]
pub struct Settlement {
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
    places_of_worship: Vec<PlaceOfWorship>,

    recent_history: RecentHistory,
    politics: Politics,
    events: Event,
    opportunity: Opportunities,
    weather: Weather,
    danger_level: DangerLevel,
    danger_type: DangerType

}

fn random_origin() -> Origin {
    match roll(8) {
        1 => Origin::Accidental,
        2 => Origin::BusinessVenture,
        3 => Origin::Crossroads,
        4 => Origin::MilitaryOutpost,
        5 => Origin::NoMansLand,
        6 => Origin::Native,
        7 => Origin::OvernightStop,
        _ => Origin::WildernessExpert
    }
}

fn random_age() -> Age {
    match roll(20) {
        1..=3 => Age::Recent,
        4..=8 => Age::Established,
        9..=13 => Age::Mature,
        14..=17 => Age::Old,
        18..=19 => Age::Ancient,
        _ => Age::Unknown
    }
}

fn random_specialty() -> Specialty {
    match roll(6) {
        1 => Specialty::AtypicalShippingMethods,
        2 =>  match roll(2) {
            1 => Specialty::ExcellentAndUniqueFood,
            _ => Specialty::PlentifulAndVariedHighQualityBeverages
        },
        3 => Specialty::Hospitality,
        4 => Specialty::Information,
        5 => Specialty::PurchasingConnections,
        _ => Specialty::UnscrupulousContractors
    }
}

fn random_condition() -> Condition {
    match roll(20) {
        1..=2 => Condition::Ramshackle,
        3..=6 => Condition::Poor,
        7..=14 => Condition::Fair,
        15..=18 => Condition::Good,
        _ => Condition::Immaculate
    }
}

fn random_visitor_traffic() -> VisitorTraffic {
    match roll(5) {
        1|2 => VisitorTraffic::Vacant,
        3..=6 => VisitorTraffic::Groups,
        7..=14 => VisitorTraffic::Crowds,
        15..=18 => VisitorTraffic::Droves,
        _ => VisitorTraffic::Masses
    }
}

fn roll_visitor_traffic(value: i32) -> VisitorTraffic {
    match roll(20) + value {
        1|2 => VisitorTraffic::Vacant,
        3..=6 => VisitorTraffic::Groups,
        7..=14 => VisitorTraffic::Crowds,
        15..=18 => VisitorTraffic::Droves,
        _ => VisitorTraffic::Masses
    }
}

fn random_size() -> Size {
    match roll(20) {
        1|2 => Size::VerySmall,
        3..=6 => Size::Small,
        7..=14 => Size::Medium,
        15..=18 => Size::Large,
        _ => Size::VeryLarge
    }
}

fn roll_size(value: i32) -> Size {
    match roll(20) + value {
        1|2 => Size::VerySmall,
        3..=6 => Size::Small,
        7..=14 => Size::Medium,
        15..=18 => Size::Large,
        _ => Size::VeryLarge
    }
}

fn random_environment() -> Environment {
    match roll(8) {
        1 => Environment::Coastal,
        2 => Environment::Forest,
        3 => Environment::Mountains,
        4 => Environment::Plains,
        5 => Environment::River,
        6 => Environment::Swamp,
        7 => Environment::Underground,
        8 => Environment::Valley,
        9 => Environment::Tundra,
        _ => Environment::Desert
    }
}

fn random_resident_population() -> ResidentPopulation {
    match roll(20) {
        1|2 => ResidentPopulation::NearlyDeserted,
        3..=6 => ResidentPopulation::Sparse,
        7..=14 => ResidentPopulation::Appropriate,
        15..=18 => ResidentPopulation::Congested,
        _ => ResidentPopulation::Overwhelmed
    }
}

fn random_demographics() -> Demographics {
    match roll(20) {
        1..=5 => Demographics::One,
        6..=9 => Demographics::Two,
        10..=14 => Demographics::Normal,
        15..=17 => Demographics::Wide,
        18|19 => Demographics::HighAndLow,
        _ => Demographics::EverChanging
    }
}

fn random_disposition() -> Disposition {
    match roll(20) {
        1|2 => Disposition::Hostile,
        3..=6 => Disposition::Unfriendly,
        7..=14 => Disposition::Neutral,
        15..=18 => Disposition::Friendly,
        _ => Disposition::Open
    }
}

fn random_law_enforcement() -> LawEnforcement {
    match roll(20) {
        1|2 => LawEnforcement::None,
        3..=6 => LawEnforcement::Sheriff,
        7..=14 => LawEnforcement::SmallLocalWatch,
        15..=18 => LawEnforcement::WellEquipped,
        _ => LawEnforcement::OverwhelmingPresence
    }
}

fn random_leadership() -> Leadership {
    match roll(20) {
        1 => Leadership::NoLeader,
        2..=4 => Leadership::Hereditary,
        5..=7 => Leadership::MerchantMonarch,
        8..=10 => Leadership::UnderworldCriminal,
        11..=13 => match roll(4) {
            1 => Leadership::Plutocracy,
            2 => Leadership::Magocracy,
            3 => Leadership::Theocracy,
            _ => Leadership::Oligarchy
        },
        14..=16 => Leadership::LocalCouncil,
        17..=19 => Leadership::SingleElected,
        _ => Leadership::AnarchoSyndicalistCommune
    }
}

fn random_population_wealth() -> PopulationWealth {
    match roll(20) {
        1|2 => PopulationWealth::Destitute,
        3..=6 => PopulationWealth::Impoverished,
        7..=14 => PopulationWealth::Average,
        15..=17 => PopulationWealth::Prosperous,
        18|19 => PopulationWealth::Wealthy,
        _ => PopulationWealth::Affluent
    }
}

fn roll_population_wealth(value: i32) -> PopulationWealth {
    match roll(20) + value {
        1|2 => PopulationWealth::Destitute,
        3..=6 => PopulationWealth::Impoverished,
        7..=14 => PopulationWealth::Average,
        15..=17 => PopulationWealth::Prosperous,
        18|19 => PopulationWealth::Wealthy,
        _ => PopulationWealth::Affluent
    }
}


fn random_crime() -> Crime {
    match roll(20) {
        1|2 => Crime::Regular,
        3..=6 => Crime::Common,
        7..=14 => Crime::Average,
        15..=18 => Crime::Uncommon,
        _ => Crime::Rare
    }
}

fn roll_crime(value: i32) -> Crime {
    match roll(20) + value {
        rolled_value if rolled_value <= 2 => Crime::Regular,
        3..=6 => Crime::Common,
        7..=14 => Crime::Average,
        15..=18 => Crime::Uncommon,
        _ => Crime::Rare
    }
}

//Step 3: Places of Interest
fn random_shop() -> Shop {
    match roll(100) {
        1..=4 => Shop {name : String::from("Baker"), shop_type: ShopType::Baker},
        5..=8 => Shop {name : String::from("Butcher"), shop_type: ShopType::Butcher},
        9..=12 => Shop {name : String::from("Cooper"), shop_type: ShopType::Cooper},
        13..=16 => Shop {name : String::from("Carpenter"), shop_type: ShopType::Carpenter},
        17..=24 => Shop {name : String::from("General Store"), shop_type: ShopType::GeneralStore},
        25..=28 => Shop {name : String::from("Herbalist"), shop_type: ShopType::Herbalist},
        29..=36 => Shop {name : String::from("Smithy"), shop_type: ShopType::Smithy},
        37..=40 => Shop {name : String::from("Tailor"), shop_type: ShopType::Tailor},
        41..=44 => Shop {name : String::from("Tanner/Taxidermist"), shop_type: ShopType::TannerTaxidermist},
        45..=48 => Shop {name : String::from("Thatcher"), shop_type: ShopType::Thatcher},
        49..=52 => Shop {name : String::from("Wainwrite"), shop_type: ShopType::Wainwrite},
        53..=56 => Shop {name : String::from("Weaver"), shop_type: ShopType::Weaver},
        57..=59 => Shop {name : String::from("Alchemist"), shop_type: ShopType::Alchemist},
        60..=62 => Shop {name : String::from("Artist"), shop_type: ShopType::Artist},
        63..=65 => Shop {name : String::from("Bank & Exchange"), shop_type: ShopType::BankAndExchange},
        66..=68 => Shop {name : String::from("Cobbler"), shop_type: ShopType::Cobbler},
        69..=71 => Shop {name : String::from("Foundry/Smelting"), shop_type: ShopType::FoundrySmelting},
        72..=74 => Shop {name : String::from("Mill"), shop_type: ShopType::Mill},
        75..=77 => Shop {name : String::from("Textile"), shop_type: ShopType::Textile},
        78..=80 => Shop {name : String::from("Shipwrite"), shop_type: ShopType::Shipwrite},
        81..=82 => Shop {name : String::from("Rare Botanicals"), shop_type: ShopType::RareBotanicals},
        83..=84 => Shop {name : String::from("Luxury Furnishing"), shop_type: ShopType::LuxuryFurnishing},
        85..=86 => Shop {name : String::from("Rare Libations & Fare"), shop_type: ShopType::RareLibationsAndFare},
        87..=88 => Shop {name : String::from("Rare Trade Goods"), shop_type: ShopType::RareTradeGoods},
        89..=90 => Shop {name : String::from("Magic Shop Armor"), shop_type: ShopType::MagicShopArmor},
        91..=92 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopBooks},
        93..=94 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopClothing},
        95..=96 => Shop {name : String::from("Magic Shop Jewelry"), shop_type: ShopType::MagicShopJewelry},
        97..=98 => Shop {name : String::from("Magic Shop Weapons"), shop_type: ShopType::MagicShopWeapons},
        _=> Shop {name : String::from("Magic Shop Misellaneous"), shop_type: ShopType::MagicShopMisellaneous},
    }
}

fn random_service() -> Service {
    let help_size: HiredHelpSize = match roll(3) {
        1 => HiredHelpSize::Individual,
        2 => HiredHelpSize::Team,
        3 => HiredHelpSize::Guild,
        _ => HiredHelpSize::None
    };

    match roll(100) {
        1..=8 => Service { name: String::from("Barber"), service_type: ServiceType::Barber, hired_help_size: HiredHelpSize::None},
        9..=16 => Service { name: String::from("Bathhouse"), service_type: ServiceType::Bathhouse, hired_help_size: HiredHelpSize::None},
        17..=24 => Service { name: String::from("Doctor"), service_type: ServiceType::Doctor, hired_help_size: HiredHelpSize::None},
        25..=32 => Service { name: String::from("House Of Leisure"), service_type: ServiceType::HouseOfLeisure, hired_help_size: HiredHelpSize::None},
        33..=44 => Service { name: String::from("Inn"), service_type: ServiceType::Inn, hired_help_size: HiredHelpSize::None},
        45..=52 => Service { name: String::from("Club"), service_type: ServiceType::Club, hired_help_size: HiredHelpSize::None},
        53..=60 => Service { name: String::from("Soothsayer"), service_type: ServiceType::Soothsayer, hired_help_size: HiredHelpSize::None},
        61..=68 => Service { name: String::from("Stable"), service_type: ServiceType::Stable, hired_help_size: HiredHelpSize::None},
        69..=80 => Service { name: String::from("Tavern"), service_type: ServiceType::Tavern, hired_help_size: HiredHelpSize::None},
        81..=82 => Service { name: String::from("Brutes & Brawlers"), service_type: ServiceType::BrutesAndBrawlers, hired_help_size: help_size},
        83..=84 => Service { name: String::from("Cloak & Dagger"), service_type: ServiceType::CloakAndDagger, hired_help_size: help_size},
        85..=86 => Service { name: String::from("Bows & SLings"), service_type: ServiceType::BowsAndSLings, hired_help_size: help_size},
        87..=88 => Service { name: String::from("Scribes & Clerks"), service_type: ServiceType::ScribesAndClerks, hired_help_size: help_size},
        89..=90 => Service { name: String::from("Guides & Trackers"), service_type: ServiceType::GuidesAndTrackers, hired_help_size: help_size},
        91..=92 => Service { name: String::from("Caravan & Mount"), service_type: ServiceType::CaravanAndMount, hired_help_size: help_size},
        93..=94 => Service { name: String::from("ArcaneA cademics"), service_type: ServiceType::ArcaneAcademics, hired_help_size: help_size},
        95..=96 => Service { name: String::from("Magic Mercenaries"), service_type: ServiceType::MagicMercenaries, hired_help_size: help_size},
        97..=98 => Service { name: String::from("Priestly Guidance"), service_type: ServiceType::PriestlyGuidance, hired_help_size: help_size},
        _ => Service { name: String::from("Hands Of The Divine"), service_type: ServiceType::HandsOfTheDivine, hired_help_size: help_size}
    }
}

fn random_place_of_worship() -> PlaceOfWorship {
    let size: PlaceOfWorshipSize = match roll(20) {
        1 => PlaceOfWorshipSize::Secret,
        2..=8 => PlaceOfWorshipSize::Altar,
        9..=14 => PlaceOfWorshipSize::Oratory,
        15..=17 => PlaceOfWorshipSize::Sanctuary,
        18..=19 => PlaceOfWorshipSize::Temple,
        _=> PlaceOfWorshipSize::GreateTemple,
    };
    let fervency: Fervency = match roll(20) {
        1..=3 => Fervency::Unseen,
        4..=7 => Fervency::Quiet,
        8..=12 => Fervency::Subtle,
        13..=16 => Fervency::Moderate,
        17..=19 => Fervency::Fervent,
        _=> Fervency::Zealous,
    };
    let alignment: Alignment = match roll(10) {
        1 => Alignment::Evil,
        2..=5 => Alignment::Neutral,
        _ => Alignment::Good
    };

    PlaceOfWorship {size: size, fervency: fervency, alignment}
}
// Extra Intrigue
fn random_recent_history() -> RecentHistory {
    match roll(6) {
        1 => RecentHistory::BenevolenceOfYesteryear,
        2 => RecentHistory::Construction,
        3 => RecentHistory::CrimeCentral,
        4 => RecentHistory::Neutrality,
        5 => RecentHistory::PopularProducer,
        _ => RecentHistory::Wartorn    
    }
}

fn random_politics() -> Politics {
    match roll(6) {
        1 => Politics::BrinkOfWar,
        2 => Politics::LawlessRegion,
        3 => Politics::Embattlement,
        4 => Politics::Revolution,
        5 => Politics::Peace,
        _ => Politics::New    
    }
}

fn random_events() -> Event {
    match roll(6) {
        1 => Event::HighClassVisitor,
        2 => Event::Troublemakers,
        3 => Event::FoolsFestival,
        4 => Event::CaughtRedHanded,
        5 => Event::Strangers,
        _ => Event::PublicEmergency    
    }
}

fn random_opportunity() -> Opportunities {
    match roll(6) {
        1 => Opportunities::PoliticalIntrigue,
        2 => Opportunities::MissingPerson,
        3 => Opportunities::Monster,
        4 => Opportunities::GuardDuty,
        5 => Opportunities::Transportation,
        _ => Opportunities::Acquisition    
    }
}

fn random_weather() -> Weather {
    match roll(20) {
        1|2 => Weather::Good,
        3..=6 => Weather::Mild,
        7..=14 => Weather::Fair,
        15..=18 => Weather::Harsh,
        _ => Weather::Bad   
    }
}

fn random_danger_level() -> DangerLevel {
    match roll(20) {
        1|2 => DangerLevel::EverPresent,
        3..=6 => DangerLevel::Frequent,
        7..=14 => DangerLevel::Common,
        15..=18 => DangerLevel::Uncommon,
        _ => DangerLevel::Rare   
    }
}

fn random_danger_type() -> DangerType {
    match roll(5) {
        1 => DangerType::SuspiciousLocals,
        2 => DangerType::Raids,
        3 => DangerType::Monster,
        4 => DangerType::Environmental,
        _ => DangerType::Cult   
    }
}

// alias for random
fn roll(size: i32) -> i32 {
    rand::thread_rng().gen_range(1..=size)
}

#[derive(Debug)]
pub struct Parameters {
    pub name: Option<String>,
    pub origin: Option<String>,
    pub specialty: Option<String>,
    pub age: Option<String>,
    pub condition: Option<String>,
    pub size: Option<String>,
    pub environment: Option<String>,
    pub resident_population: Option<String>,
    pub number_shops: Option<i32>,
    pub number_services: Option<i32>,
    pub rare_magic: Option<bool>
}


//Define base settlement
pub fn default_settlement() -> Settlement {
    Settlement {
        name: String::from("default"),
        origin: Origin::Accidental,
        specialty: Specialty::AtypicalShippingMethods,
        age: Age::Unknown,
        condition: Condition::Ramshackle,
        visitor_traffic: VisitorTraffic::Vacant,
        size: Size::VerySmall,
        environment: Environment::Plains,
        resident_population: ResidentPopulation::Sparse,
        demographics: Demographics::One,
        disposition: Disposition::Neutral,
        law_enforcement: LawEnforcement::None,
        leadership: Leadership::NoLeader,
        population_wealth: PopulationWealth::Average,
        crime: Crime::Average,
        shops: Vec::new(),
        places_of_worship: Vec::new(),
        services: Vec::new(),
        recent_history: RecentHistory::Wartorn,
        politics: Politics::BrinkOfWar,
        events: Event::PublicEmergency,
        opportunity: Opportunities::Monster,
        weather: Weather::Fair,
        danger_level: DangerLevel::Common,
        danger_type: DangerType::Monster

    }

}


pub fn generate_settlement (par: Parameters) -> Settlement {
    let mut settlement: Settlement = default_settlement();
    //Step 1: Basics
    settlement.origin = random_origin();
    settlement.specialty = random_specialty();
    if let Specialty::UnscrupulousContractors = settlement.specialty {
        let help_size: HiredHelpSize = match roll(3) {
            1 => HiredHelpSize::Individual,
            2 => HiredHelpSize::Team,
            3 => HiredHelpSize::Guild,
            _ => HiredHelpSize::None
        };

        match roll(10) {
            1 => settlement.services.push(Service {name: String::from("Brutes and Brawlers"), service_type: ServiceType::BrutesAndBrawlers, hired_help_size: help_size}),
            2 => settlement.services.push(Service {name: String::from("Cloak and Dagger"), service_type: ServiceType::CloakAndDagger, hired_help_size: help_size}),
            3 => settlement.services.push(Service {name: String::from("Bows and Slings"), service_type: ServiceType::BowsAndSLings, hired_help_size: help_size}),
            4 => settlement.services.push(Service {name: String::from("Scribes and Clerks"), service_type: ServiceType::ScribesAndClerks, hired_help_size: help_size}),
            5 => settlement.services.push(Service {name: String::from("Guides and Trackers"), service_type: ServiceType::GuidesAndTrackers, hired_help_size: help_size}),
            6 => settlement.services.push(Service {name: String::from("Caravan and Mount"), service_type: ServiceType::CaravanAndMount, hired_help_size: help_size}),
            7 => settlement.services.push(Service {name: String::from("Arcane Academics"), service_type: ServiceType::ArcaneAcademics, hired_help_size: help_size}),
            8 => settlement.services.push(Service {name: String::from("Magic Mercenaries"), service_type: ServiceType::MagicMercenaries, hired_help_size: help_size}),
            9 => settlement.services.push(Service {name: String::from("Priestly Guidance"), service_type: ServiceType::PriestlyGuidance, hired_help_size: help_size}),
            _ => settlement.services.push(Service {name: String::from("Hands of the Divine"), service_type: ServiceType::HandsOfTheDivine, hired_help_size: help_size})   
        }
    }
    settlement.age = random_age();
    let visitor_traffic_modifier = match settlement.age {
        Age::Recent => -1,
        Age::Established => 0,
        Age::Mature => 1,
        Age::Old => 2,
        Age::Ancient => 3,
        Age::Unknown => 4
    };

    settlement.condition = random_condition();
    let population_wealth_modifier = match settlement.condition {
        Condition::Ramshackle => -6,
        Condition::Poor => -3,
        Condition::Fair => 0,
        Condition::Good => 3,
        Condition::Immaculate => 6
    };

    settlement.visitor_traffic = roll_visitor_traffic(visitor_traffic_modifier);

    let mut size_modifier = 0;
    let mut crime_modifier = 0;
    match settlement.visitor_traffic {
        VisitorTraffic::Vacant => {
            size_modifier = 0;
            crime_modifier = 2;
        },
        VisitorTraffic::Groups => {
            size_modifier = 1;
            crime_modifier = 1;
        },
        VisitorTraffic::Crowds => {
            size_modifier = 2;
            crime_modifier = 0;
        },
        VisitorTraffic::Droves => {
            size_modifier = 3;
            crime_modifier = -1;
        },
        VisitorTraffic::Masses => {
            size_modifier = 4;
            crime_modifier = -2;
        },
    };

    settlement.size = roll_size(size_modifier);
    settlement.environment = random_environment();
    //Step 2: Community
    settlement.resident_population = random_resident_population();
    crime_modifier = match settlement.resident_population {
        ResidentPopulation::NearlyDeserted => crime_modifier + 2,
        ResidentPopulation::Sparse => crime_modifier + 1,
        ResidentPopulation::Appropriate => crime_modifier,
        ResidentPopulation::Congested => crime_modifier - 1,
        ResidentPopulation::Overwhelmed => crime_modifier - 2
    };

    settlement.demographics = random_demographics();
    settlement.disposition = random_disposition();

    settlement.law_enforcement = random_law_enforcement();
    crime_modifier = match settlement.law_enforcement {
        LawEnforcement::None => crime_modifier - 8,
        LawEnforcement::Sheriff => crime_modifier - 4,
        LawEnforcement::SmallLocalWatch => crime_modifier,
        LawEnforcement::WellEquipped => crime_modifier + 4,
        LawEnforcement::OverwhelmingPresence => crime_modifier + 8,
    };

    settlement.leadership = random_leadership();
    settlement.population_wealth = roll_population_wealth(population_wealth_modifier);

    let mut quality_modifier = 0;
    match settlement.population_wealth {
        PopulationWealth::Destitute => {
            crime_modifier = crime_modifier -4;
            quality_modifier = -2;
        },
        PopulationWealth::Impoverished => {
            crime_modifier = crime_modifier -2;
            quality_modifier = -1;
        },
        PopulationWealth::Average => {
            crime_modifier = crime_modifier;
            quality_modifier = 0;
        },
        PopulationWealth::Prosperous => {
            crime_modifier = crime_modifier -1;
            quality_modifier = 0;
        },
        PopulationWealth::Wealthy => {
            crime_modifier = crime_modifier -2;
            quality_modifier = 2;
        },
        PopulationWealth::Affluent => {
            crime_modifier = crime_modifier -4;
            quality_modifier = 3;
        },
    }

    settlement.crime = roll_crime(crime_modifier);

    //Step 3: Points of Interest
    let number_of_shops = match settlement.size {
        Size::VerySmall => roll(8) + 2,
        Size::Small => roll(8) + 4,
        Size::Medium => roll(8) + 6,
        Size::Large => roll(8) + 8,
        Size::VeryLarge => roll(8) + 10
    };

    for _ in 0..number_of_shops {
        settlement.shops.push(random_shop());
    };
    
    let mut number_of_services = match settlement.size {
        Size::VerySmall => roll(6),
        Size::Small => roll(6) + 1,
        Size::Medium => roll(6) + 3,
        Size::Large => roll(6) + 5,
        Size::VeryLarge => roll(6) + 7
    };

    if settlement.services.len() > 1 {
        number_of_services = number_of_services - 1;
    }

    for _ in 0..number_of_services {
        settlement.services.push(random_service());
    }

    if  roll(2) == 2 {
        settlement.places_of_worship.push(random_place_of_worship())
    }
    
    //Extra Intrigue
    settlement.recent_history = random_recent_history();
    settlement.politics = random_politics();
    settlement.events = random_events();
    settlement.opportunity = random_opportunity();
    settlement.danger_level = random_danger_level();
    settlement.danger_type = random_danger_type();


    match par.name {
        Some(val) => settlement.name = val,
        None => settlement.name = format!("{:?} Place", settlement.environment)
    }
    

    settlement
}
