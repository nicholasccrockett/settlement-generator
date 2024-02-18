use strum_macros::EnumString;
use std::str::FromStr;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod trading_post;
pub mod village;
pub mod town;
pub mod capitol;
pub mod city;
pub mod fortress;

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Quality {
    Poor,
    Good,
    Fine
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Alignment {
    Evil,
    Neutral,
    Good
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
    pub demographics: Option<String>,
    pub disposition: Option<String>,
    pub law_enforcement: Option<String>,
    pub leadership: Option<String>,
    pub population_wealth: Option<String>,
    pub number_shops: Option<i32>,
    pub number_services: Option<i32>,
    pub rare_magic: Option<bool>
}
impl Parameters {
    pub fn empty() -> Parameters {
        return Parameters{ name: None, origin: None, specialty: None, age: None, condition: None, size: None, environment: None, resident_population: None, demographics: None, disposition: None, law_enforcement: None, leadership: None, population_wealth: None, number_shops: None, number_services: None, rare_magic: None};
    }

    pub fn build( name: Option<String>, origin: Option<String>, specialty: Option<String>, age: Option<String>, condition: Option<String>, size: Option<String>, environment: Option<String>, resident_population: Option<String>, demographics: Option<String>, disposition: Option<String>, law_enforcement: Option<String>, leadership: Option<String>, population_wealth: Option<String>, number_shops: Option<i32>, number_services: Option<i32>,
        rare_magic: Option<bool>) -> Parameters{
            return Parameters { name, origin, specialty, age, condition, size, environment, resident_population, demographics, disposition, law_enforcement, leadership, population_wealth, number_shops, number_services, rare_magic }
        }

}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum SettlementType {
    TradingPost,
    Village,
    Town,
    City,
    Capitol,
    Fortress
}
impl SettlementType {
    fn random() -> SettlementType {
        match roll(6) {
            1 => SettlementType::TradingPost,
            2 => SettlementType::Village,
            3 => SettlementType::Town,
            4 => SettlementType::City,
            5 => SettlementType::Capitol,
            6 => SettlementType::Fortress,
            _ => panic!("SettlementType Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
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
impl Origin {
    fn random() -> Origin {
        match roll(8) {
            1 => Origin::Accidental,
            2 => Origin::BusinessVenture,
            3 => Origin::Crossroads,
            4 => Origin::MilitaryOutpost,
            5 => Origin::NoMansLand,
            6 => Origin::Native,
            7 => Origin::OvernightStop,
            8 => Origin::WildernessExpert,
            _ => panic!("Origin Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Age {
    Recent,
    Established,
    Mature,
    Old,
    Ancient,
    Unknown
}
impl Age {
    fn trading_post_random() -> Age {
        match roll(20) {
            1..=3 => Age::Recent,
            4..=8 => Age::Established,
            9..=13 => Age::Mature,
            14..=17 => Age::Old,
            18..=19 => Age::Ancient,
            _ => Age::Unknown
        }
    }

    fn village_random() -> Age {
        match roll(20) {
            1..=5 => Age::Recent,
            6..=10 => Age::Established,
            11..=14 => Age::Mature,
            15..=18 => Age::Old,
            19..=20 => Age::Ancient,
            _ => panic!("Age Roll Error")
        }
    }

    fn random() -> Age {
        match roll(5) {
            1 => Age::Recent,
            2 => Age::Established,
            3 => Age::Mature,
            4 => Age::Old,
            5 => Age::Ancient,
            _ => panic!("Age Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Specialty {
    AtypicalShippingMethods,
    ExcellentAndUniqueFood,
    PlentifulAndVariedHighQualityBeverages,
    Hospitality,
    Information,
    PurchasingConnections,
    UnscrupulousContractors
}
impl Specialty {
    fn random() -> Specialty {
        match roll(6) {
            1 => Specialty::AtypicalShippingMethods,
            2 =>  match roll(2) {
                1 => Specialty::ExcellentAndUniqueFood,
                _ => Specialty::PlentifulAndVariedHighQualityBeverages
            },
            3 => Specialty::Hospitality,
            4 => Specialty::Information,
            5 => Specialty::PurchasingConnections,
            6 => Specialty::UnscrupulousContractors,
            _ => panic!("Specialty Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Condition {
    Ramshackle,
    Poor,
    Fair,
    Good,
    Immaculate
}
impl Condition {
    fn random() -> Condition {
        match roll(20) {
            1..=2 => Condition::Ramshackle,
            3..=6 => Condition::Poor,
            7..=14 => Condition::Fair,
            15..=18 => Condition::Good,
            19..=20 => Condition::Immaculate,
            _ => panic!("Condition Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum VisitorTraffic {
    Vacant,
    Groups,
    Crowds,
    Droves,
    Masses
}
impl VisitorTraffic {
    fn random(value: i32) -> VisitorTraffic {
        match roll(20) + value {
            1|2 => VisitorTraffic::Vacant,
            3..=6 => VisitorTraffic::Groups,
            7..=14 => VisitorTraffic::Crowds,
            15..=18 => VisitorTraffic::Droves,
            19..=20 => VisitorTraffic::Masses,
            _ => panic!("VisitorTraffic Roll Error")
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Size {
    VerySmall,
    Small,
    Medium,
    Large,
    VeryLarge
}
impl Size {
    fn random(value: i32) -> Size {
        match roll(20) + value {
            1|2 => Size::VerySmall,
            3..=6 => Size::Small,
            7..=14 => Size::Medium,
            15..=18 => Size::Large,
            _ => Size::VeryLarge
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
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
impl Environment {
    fn random() -> Environment {
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
            10 => Environment::Desert,
            _ => panic!()
        }
    }
}

//Step 2:Community
#[derive(Debug,Serialize,Deserialize,EnumString)]
enum ResidentPopulation {
    NearlyDeserted,
    Sparse,
    Appropriate,
    Congested,
    Overwhelmed
}
impl ResidentPopulation {
    fn random() -> ResidentPopulation {
        match roll(20) {
            1|2 => ResidentPopulation::NearlyDeserted,
            3..=6 => ResidentPopulation::Sparse,
            7..=14 => ResidentPopulation::Appropriate,
            15..=18 => ResidentPopulation::Congested,
            _ => ResidentPopulation::Overwhelmed
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Demographics {
    One,
    Two,
    Normal,
    Wide,
    HighAndLow,
    EverChanging
}
impl Demographics {
    fn random() -> Demographics {
        match roll(20) {
            1..=5 => Demographics::One,
            6..=9 => Demographics::Two,
            10..=14 => Demographics::Normal,
            15..=17 => Demographics::Wide,
            18|19 => Demographics::HighAndLow,
            _ => Demographics::EverChanging
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Disposition {
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Open
}
impl Disposition {
    fn random() -> Disposition {
        match roll(20) {
            1|2 => Disposition::Hostile,
            3..=6 => Disposition::Unfriendly,
            7..=14 => Disposition::Neutral,
            15..=18 => Disposition::Friendly,
            _ => Disposition::Open
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum LawEnforcement {
    None,
    Sheriff,
    SmallLocalWatch,
    WellEquipped,
    OverwhelmingPresence
}
impl LawEnforcement {
    fn random() -> LawEnforcement {
        match roll(20) {
            1|2 => LawEnforcement::None,
            3..=6 => LawEnforcement::Sheriff,
            7..=14 => LawEnforcement::SmallLocalWatch,
            15..=18 => LawEnforcement::WellEquipped,
            _ => LawEnforcement::OverwhelmingPresence
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
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
impl Leadership {
    fn random() -> Leadership {
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
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum PopulationWealth {
    Destitute,
    Impoverished,
    Average,
    Prosperous,
    Wealthy,
    Affluent
}
impl PopulationWealth {
    fn random(value: i32) -> PopulationWealth {
        match roll(20) + value {
            1|2 => PopulationWealth::Destitute,
            3..=6 => PopulationWealth::Impoverished,
            7..=14 => PopulationWealth::Average,
            15..=17 => PopulationWealth::Prosperous,
            18|19 => PopulationWealth::Wealthy,
            _ => PopulationWealth::Affluent
        }
    }
    
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Crime {
    Regular,
    Common,
    Average,
    Uncommon,
    Rare
}
impl Crime {
    fn roll_crime(value: i32) -> Crime {
        match roll(20) + value {
            rolled_value if rolled_value <= 2 => Crime::Regular,
            3..=6 => Crime::Common,
            7..=14 => Crime::Average,
            15..=18 => Crime::Uncommon,
            _ => Crime::Rare
        }
    }
}

//Step 3: Points of Interest
#[derive(Debug,Serialize,Deserialize,EnumString)]
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
impl ShopType {
    fn random(val: i32) -> Shop {
        let quality = match roll(12) + val {
            rolled_value if rolled_value <= 4 => Quality::Poor,
            5..=10 => Quality::Fine,
            _ => Quality::Good
        };
    
        match roll(100) {
            1..=4 => Shop {name : String::from("Baker"), shop_type: ShopType::Baker, quality: quality},
            5..=8 => Shop {name : String::from("Butcher"), shop_type: ShopType::Butcher, quality: quality},
            9..=12 => Shop {name : String::from("Cooper"), shop_type: ShopType::Cooper, quality: quality},
            13..=16 => Shop {name : String::from("Carpenter"), shop_type: ShopType::Carpenter, quality: quality},
            17..=24 => Shop {name : String::from("General Store"), shop_type: ShopType::GeneralStore, quality: quality},
            25..=28 => Shop {name : String::from("Herbalist"), shop_type: ShopType::Herbalist, quality: quality},
            29..=36 => Shop {name : String::from("Smithy"), shop_type: ShopType::Smithy, quality: quality},
            37..=40 => Shop {name : String::from("Tailor"), shop_type: ShopType::Tailor, quality: quality},
            41..=44 => Shop {name : String::from("Tanner/Taxidermist"), shop_type: ShopType::TannerTaxidermist, quality: quality},
            45..=48 => Shop {name : String::from("Thatcher"), shop_type: ShopType::Thatcher, quality: quality},
            49..=52 => Shop {name : String::from("Wainwrite"), shop_type: ShopType::Wainwrite, quality: quality},
            53..=56 => Shop {name : String::from("Weaver"), shop_type: ShopType::Weaver, quality: quality},
            57..=59 => Shop {name : String::from("Alchemist"), shop_type: ShopType::Alchemist, quality: quality},
            60..=62 => Shop {name : String::from("Artist"), shop_type: ShopType::Artist, quality: quality},
            63..=65 => Shop {name : String::from("Bank & Exchange"), shop_type: ShopType::BankAndExchange, quality: quality},
            66..=68 => Shop {name : String::from("Cobbler"), shop_type: ShopType::Cobbler, quality: quality},
            69..=71 => Shop {name : String::from("Foundry/Smelting"), shop_type: ShopType::FoundrySmelting, quality: quality},
            72..=74 => Shop {name : String::from("Mill"), shop_type: ShopType::Mill, quality: quality},
            75..=77 => Shop {name : String::from("Textile"), shop_type: ShopType::Textile, quality: quality},
            78..=80 => Shop {name : String::from("Shipwrite"), shop_type: ShopType::Shipwrite, quality: quality},
            81..=82 => Shop {name : String::from("Rare Botanicals"), shop_type: ShopType::RareBotanicals, quality: quality},
            83..=84 => Shop {name : String::from("Luxury Furnishing"), shop_type: ShopType::LuxuryFurnishing, quality: quality},
            85..=86 => Shop {name : String::from("Rare Libations & Fare"), shop_type: ShopType::RareLibationsAndFare, quality: quality},
            87..=88 => Shop {name : String::from("Rare Trade Goods"), shop_type: ShopType::RareTradeGoods, quality: quality},
            89..=90 => Shop {name : String::from("Magic Shop Armor"), shop_type: ShopType::MagicShopArmor, quality: quality},
            91..=92 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopBooks, quality: quality},
            93..=94 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopClothing, quality: quality},
            95..=96 => Shop {name : String::from("Magic Shop Jewelry"), shop_type: ShopType::MagicShopJewelry, quality: quality},
            97..=98 => Shop {name : String::from("Magic Shop Weapons"), shop_type: ShopType::MagicShopWeapons, quality: quality},
            _=> Shop {name : String::from("Magic Shop Misellaneous"), shop_type: ShopType::MagicShopMisellaneous, quality: quality},
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
struct Shop {
    name: String,
    shop_type: ShopType,
    quality: Quality
}
impl Shop {
    fn random(val: i32) -> Shop {
        let quality = match roll(12) + val {
            rolled_value if rolled_value <= 4 => Quality::Poor,
            5..=10 => Quality::Fine,
            _ => Quality::Good
        };
    
        match roll(100) {
            1..=4 => Shop {name : String::from("Baker"), shop_type: ShopType::Baker, quality: quality},
            5..=8 => Shop {name : String::from("Butcher"), shop_type: ShopType::Butcher, quality: quality},
            9..=12 => Shop {name : String::from("Cooper"), shop_type: ShopType::Cooper, quality: quality},
            13..=16 => Shop {name : String::from("Carpenter"), shop_type: ShopType::Carpenter, quality: quality},
            17..=24 => Shop {name : String::from("General Store"), shop_type: ShopType::GeneralStore, quality: quality},
            25..=28 => Shop {name : String::from("Herbalist"), shop_type: ShopType::Herbalist, quality: quality},
            29..=36 => Shop {name : String::from("Smithy"), shop_type: ShopType::Smithy, quality: quality},
            37..=40 => Shop {name : String::from("Tailor"), shop_type: ShopType::Tailor, quality: quality},
            41..=44 => Shop {name : String::from("Tanner/Taxidermist"), shop_type: ShopType::TannerTaxidermist, quality: quality},
            45..=48 => Shop {name : String::from("Thatcher"), shop_type: ShopType::Thatcher, quality: quality},
            49..=52 => Shop {name : String::from("Wainwrite"), shop_type: ShopType::Wainwrite, quality: quality},
            53..=56 => Shop {name : String::from("Weaver"), shop_type: ShopType::Weaver, quality: quality},
            57..=59 => Shop {name : String::from("Alchemist"), shop_type: ShopType::Alchemist, quality: quality},
            60..=62 => Shop {name : String::from("Artist"), shop_type: ShopType::Artist, quality: quality},
            63..=65 => Shop {name : String::from("Bank & Exchange"), shop_type: ShopType::BankAndExchange, quality: quality},
            66..=68 => Shop {name : String::from("Cobbler"), shop_type: ShopType::Cobbler, quality: quality},
            69..=71 => Shop {name : String::from("Foundry/Smelting"), shop_type: ShopType::FoundrySmelting, quality: quality},
            72..=74 => Shop {name : String::from("Mill"), shop_type: ShopType::Mill, quality: quality},
            75..=77 => Shop {name : String::from("Textile"), shop_type: ShopType::Textile, quality: quality},
            78..=80 => Shop {name : String::from("Shipwrite"), shop_type: ShopType::Shipwrite, quality: quality},
            81..=82 => Shop {name : String::from("Rare Botanicals"), shop_type: ShopType::RareBotanicals, quality: quality},
            83..=84 => Shop {name : String::from("Luxury Furnishing"), shop_type: ShopType::LuxuryFurnishing, quality: quality},
            85..=86 => Shop {name : String::from("Rare Libations & Fare"), shop_type: ShopType::RareLibationsAndFare, quality: quality},
            87..=88 => Shop {name : String::from("Rare Trade Goods"), shop_type: ShopType::RareTradeGoods, quality: quality},
            89..=90 => Shop {name : String::from("Magic Shop Armor"), shop_type: ShopType::MagicShopArmor, quality: quality},
            91..=92 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopBooks, quality: quality},
            93..=94 => Shop {name : String::from("Magic Shop Books"), shop_type: ShopType::MagicShopClothing, quality: quality},
            95..=96 => Shop {name : String::from("Magic Shop Jewelry"), shop_type: ShopType::MagicShopJewelry, quality: quality},
            97..=98 => Shop {name : String::from("Magic Shop Weapons"), shop_type: ShopType::MagicShopWeapons, quality: quality},
            _=> Shop {name : String::from("Magic Shop Misellaneous"), shop_type: ShopType::MagicShopMisellaneous, quality: quality},
        }
    }
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum HiredHelpSize {
    Individual,
    Team,
    Guild,
    None
}
impl HiredHelpSize {
    fn random() -> HiredHelpSize {
        return match roll(3) {
        1 => HiredHelpSize::Individual,
        2 => HiredHelpSize::Team,
        3 => HiredHelpSize::Guild,
        4 => HiredHelpSize::None,
        _ => panic!()
        };
    }
}

#[derive(Debug,Serialize,Deserialize)]
struct Service {
    name: String,
    service_type: ServiceType,
    hired_help_size: HiredHelpSize
}
impl Service {
    fn random() -> Service {
        let help_size:HiredHelpSize = HiredHelpSize::random();
    
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

    fn roll_num_services(size: &Size) -> i32 {
        roll(6) + match size {
            Size::VerySmall => 2,
            Size::Small => 4,
            Size::Medium => 6,
            Size::Large => 8,
            Size::VeryLarge => 10
            }
    }
}

#[derive(Debug,Serialize,Deserialize)]
struct PlaceOfWorship {
    size: PlaceOfWorshipSize,
    fervency: Fervency,
    alignment: Alignment
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum Fervency {
    Unseen,
    Quiet,
    Subtle,
    Moderate,
    Fervent,
    Zealous
}

#[derive(Debug,Serialize,Deserialize,EnumString)]
enum PlaceOfWorshipSize {
    Secret,
    Altar,
    Oratory,
    Sanctuary,
    Temple,
    GreateTemple
}


#[derive(Debug,Serialize,Deserialize,EnumString)]
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
impl ServiceType {
    
}

pub fn roll(size: i32) -> i32 {
    rand::thread_rng().gen_range(1..=size)
}