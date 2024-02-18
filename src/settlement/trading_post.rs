use crate::settlement::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingPost {
    settlement_type: SettlementType,

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
    danger_type: DangerType,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Crime {
    Regular,
    Common,
    Average,
    Uncommon,
    Rare,
}
impl Crime {
    fn random(value: i32) -> Crime {
        match roll(20) + value {
            rolled_value if rolled_value <= 2 => Crime::Regular,
            3..=6 => Crime::Common,
            7..=14 => Crime::Average,
            15..=18 => Crime::Uncommon,
            _ => Crime::Rare,
        }
    }
}

fn roll_num_shops(size: &Size) -> i32 {
    roll(8)
        + match size {
            Size::VerySmall => 2,
            Size::Small => 4,
            Size::Medium => 6,
            Size::Large => 8,
            Size::VeryLarge => 10,
        }
}

fn roll_num_services(size: &Size) -> i32 {
    roll(6)
        + match size {
            Size::VerySmall => 2,
            Size::Small => 4,
            Size::Medium => 6,
            Size::Large => 8,
            Size::VeryLarge => 10,
        }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum PlaceOfWorshipSize {
    Secret,
    Altar,
    Oratory,
    Sanctuary,
    Temple,
    GreateTemple,
}
#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Fervency {
    Unseen,
    Quiet,
    Subtle,
    Moderate,
    Fervent,
    Zealous,
}
#[derive(Debug, Serialize, Deserialize)]

struct PlaceOfWorship {
    size: PlaceOfWorshipSize,
    fervency: Fervency,
    alignment: Alignment,
}
impl PlaceOfWorship {
    fn random() -> PlaceOfWorship {
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
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum RecentHistory {
    BenevolenceOfYesteryear,
    Construction,
    CrimeCentral,
    Neutrality,
    PopularProducer,
    Wartorn,
}
impl RecentHistory {
    fn random() -> RecentHistory {
        match roll(6) {
            1 => RecentHistory::BenevolenceOfYesteryear,
            2 => RecentHistory::Construction,
            3 => RecentHistory::CrimeCentral,
            4 => RecentHistory::Neutrality,
            5 => RecentHistory::PopularProducer,
            _ => RecentHistory::Wartorn    
        }
    }
}


#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Politics {
    BrinkOfWar,
    LawlessRegion,
    Embattlement,
    Revolution,
    Peace,
    New,
}
impl Politics {
    fn random() -> Politics {
        match roll(6) {
            1 => Politics::BrinkOfWar,
            2 => Politics::LawlessRegion,
            3 => Politics::Embattlement,
            4 => Politics::Revolution,
            5 => Politics::Peace,
            _ => Politics::New    
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Event {
    HighClassVisitor,
    Troublemakers,
    FoolsFestival,
    CaughtRedHanded,
    Strangers,
    PublicEmergency,
}
impl Event {
    fn random() -> Event {
        match roll(6) {
            1 => Event::HighClassVisitor,
            2 => Event::Troublemakers,
            3 => Event::FoolsFestival,
            4 => Event::CaughtRedHanded,
            5 => Event::Strangers,
            _ => Event::PublicEmergency    
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Opportunities {
    PoliticalIntrigue,
    MissingPerson,
    Monster,
    GuardDuty,
    Transportation,
    Acquisition,
}
impl Opportunities {
    fn random() -> Opportunities {
        match roll(6) {
            1 => Opportunities::PoliticalIntrigue,
            2 => Opportunities::MissingPerson,
            3 => Opportunities::Monster,
            4 => Opportunities::GuardDuty,
            5 => Opportunities::Transportation,
            _ => Opportunities::Acquisition    
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum Weather {
    Good,
    Mild,
    Fair,
    Harsh,
    Bad,
}
impl Weather {
    fn random() -> Weather {
        match roll(20) {
            1|2 => Weather::Good,
            3..=6 => Weather::Mild,
            7..=14 => Weather::Fair,
            15..=18 => Weather::Harsh,
            _ => Weather::Bad   
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum DangerLevel {
    EverPresent,
    Frequent,
    Common,
    Uncommon,
    Rare,
}
impl DangerLevel {
    fn random() -> DangerLevel {
        match roll(20) {
            1|2 => DangerLevel::EverPresent,
            3..=6 => DangerLevel::Frequent,
            7..=14 => DangerLevel::Common,
            15..=18 => DangerLevel::Uncommon,
            _ => DangerLevel::Rare   
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
enum DangerType {
    SuspiciousLocals,
    Raids,
    Monster,
    Environmental,
    Cult,
}
impl DangerType {
    fn random() -> DangerType {
        match roll(5) {
            1 => DangerType::SuspiciousLocals,
            2 => DangerType::Raids,
            3 => DangerType::Monster,
            4 => DangerType::Environmental,
            _ => DangerType::Cult   
        }
    }
    
}

impl TradingPost {
    pub fn default() -> TradingPost {
        return TradingPost {
            settlement_type: SettlementType::TradingPost,
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
            danger_type: DangerType::Monster,
        };
    }

    // Trading Post
    pub fn generate(par: Parameters) -> TradingPost {
        let mut trading_post: TradingPost = TradingPost::default();
        //Step 1: Basics
        trading_post.origin = match par.origin {
            Some(val) => Origin::from_str(&val).expect(format!("Given value does not match any origin: {}", String::from(val)).as_str()),
            None => Origin::random(),
        };
        trading_post.specialty = match par.specialty {
            Some(val) => Specialty::from_str(&val).unwrap(),
            None => Specialty::random(),
        };

        if let Specialty::UnscrupulousContractors = trading_post.specialty {
            let help_size: HiredHelpSize = match roll(3) {
                1 => HiredHelpSize::Individual,
                2 => HiredHelpSize::Team,
                3 => HiredHelpSize::Guild,
                _ => HiredHelpSize::None,
            };

            match roll(10) {
                1 => trading_post.services.push(Service {
                    name: String::from("Brutes and Brawlers"),
                    service_type: ServiceType::BrutesAndBrawlers,
                    hired_help_size: help_size,
                }),
                2 => trading_post.services.push(Service {
                    name: String::from("Cloak and Dagger"),
                    service_type: ServiceType::CloakAndDagger,
                    hired_help_size: help_size,
                }),
                3 => trading_post.services.push(Service {
                    name: String::from("Bows and Slings"),
                    service_type: ServiceType::BowsAndSLings,
                    hired_help_size: help_size,
                }),
                4 => trading_post.services.push(Service {
                    name: String::from("Scribes and Clerks"),
                    service_type: ServiceType::ScribesAndClerks,
                    hired_help_size: help_size,
                }),
                5 => trading_post.services.push(Service {
                    name: String::from("Guides and Trackers"),
                    service_type: ServiceType::GuidesAndTrackers,
                    hired_help_size: help_size,
                }),
                6 => trading_post.services.push(Service {
                    name: String::from("Caravan and Mount"),
                    service_type: ServiceType::CaravanAndMount,
                    hired_help_size: help_size,
                }),
                7 => trading_post.services.push(Service {
                    name: String::from("Arcane Academics"),
                    service_type: ServiceType::ArcaneAcademics,
                    hired_help_size: help_size,
                }),
                8 => trading_post.services.push(Service {
                    name: String::from("Magic Mercenaries"),
                    service_type: ServiceType::MagicMercenaries,
                    hired_help_size: help_size,
                }),
                9 => trading_post.services.push(Service {
                    name: String::from("Priestly Guidance"),
                    service_type: ServiceType::PriestlyGuidance,
                    hired_help_size: help_size,
                }),
                _ => trading_post.services.push(Service {
                    name: String::from("Hands of the Divine"),
                    service_type: ServiceType::HandsOfTheDivine,
                    hired_help_size: help_size,
                }),
            }
        };
        trading_post.age = match par.age {
            Some(val) => Age::from_str(&val).unwrap(),
            None => Age::trading_post_random(),
        };
        let visitor_traffic_modifier = match trading_post.age {
            Age::Recent => -1,
            Age::Established => 0,
            Age::Mature => 1,
            Age::Old => 2,
            Age::Ancient => 3,
            Age::Unknown => 4,
        };

        trading_post.condition = match par.condition {
            Some(val) => Condition::from_str(&val).unwrap(),
            None => Condition::random(),
        };
        let population_wealth_modifier = match trading_post.condition {
            Condition::Ramshackle => -6,
            Condition::Poor => -3,
            Condition::Fair => 0,
            Condition::Good => 3,
            Condition::Immaculate => 6,
        };

        trading_post.visitor_traffic = VisitorTraffic::random(visitor_traffic_modifier);

        let mut size_modifier = 0;
        let mut crime_modifier = 0;
        match trading_post.visitor_traffic {
            VisitorTraffic::Vacant => {
                size_modifier = 0;
                crime_modifier = 2;
            }
            VisitorTraffic::Groups => {
                size_modifier = 1;
                crime_modifier = 1;
            }
            VisitorTraffic::Crowds => {
                size_modifier = 2;
                crime_modifier = 0;
            }
            VisitorTraffic::Droves => {
                size_modifier = 3;
                crime_modifier = -1;
            }
            VisitorTraffic::Masses => {
                size_modifier = 4;
                crime_modifier = -2;
            }
        };

        trading_post.size = match par.size {
            Some(val) => Size::from_str(&val).unwrap(),
            None => Size::random(size_modifier),
        };
        // trading_post.size = roll_size(size_modifier);
        trading_post.environment = match par.environment {
            Some(val) => Environment::from_str(&val).unwrap(),
            None => Environment::random(),
        };
        // trading_post.environment = random_environment();
        //Step 2: Community
        trading_post.resident_population = match par.resident_population {
            Some(val) => ResidentPopulation::from_str(&val).unwrap(),
            None => ResidentPopulation::random(),
        };
        // trading_post.resident_population = random_resident_population();
        crime_modifier = match trading_post.resident_population {
            ResidentPopulation::NearlyDeserted => crime_modifier + 2,
            ResidentPopulation::Sparse => crime_modifier + 1,
            ResidentPopulation::Appropriate => crime_modifier,
            ResidentPopulation::Congested => crime_modifier - 1,
            ResidentPopulation::Overwhelmed => crime_modifier - 2,
        };

        trading_post.demographics = match par.demographics {
            Some(val) => Demographics::from_str(&val).unwrap(),
            None => Demographics::random(),
        };
        // trading_post.demographics = random_demographics();
        trading_post.disposition = match par.disposition {
            Some(val) => Disposition::from_str(&val).unwrap(),
            None => Disposition::random(),
        };
        // trading_post.disposition = random_disposition();
        trading_post.law_enforcement = match par.law_enforcement {
            Some(val) => LawEnforcement::from_str(&val).unwrap(),
            None => LawEnforcement::random(),
        };
        // trading_post.law_enforcement = random_law_enforcement();
        crime_modifier = match trading_post.law_enforcement {
            LawEnforcement::None => crime_modifier - 8,
            LawEnforcement::Sheriff => crime_modifier - 4,
            LawEnforcement::SmallLocalWatch => crime_modifier,
            LawEnforcement::WellEquipped => crime_modifier + 4,
            LawEnforcement::OverwhelmingPresence => crime_modifier + 8,
        };

        trading_post.leadership = match par.leadership {
            Some(val) => Leadership::from_str(&val).unwrap(),
            None => Leadership::random(),
        };
        // trading_post.leadership = random_leadership();
        trading_post.population_wealth = match par.population_wealth {
            Some(val) => PopulationWealth::from_str(&val).unwrap(),
            None => PopulationWealth::random(population_wealth_modifier),
        };
        // trading_post.population_wealth = roll_population_wealth(population_wealth_modifier);

        let mut quality_modifier = 0;
        match trading_post.population_wealth {
            PopulationWealth::Destitute => {
                crime_modifier = crime_modifier - 4;
                quality_modifier = -2;
            }
            PopulationWealth::Impoverished => {
                crime_modifier = crime_modifier - 2;
                quality_modifier = -1;
            }
            PopulationWealth::Average => {
                crime_modifier = crime_modifier;
                quality_modifier = 0;
            }
            PopulationWealth::Prosperous => {
                crime_modifier = crime_modifier - 1;
                quality_modifier = 0;
            }
            PopulationWealth::Wealthy => {
                crime_modifier = crime_modifier - 2;
                quality_modifier = 2;
            }
            PopulationWealth::Affluent => {
                crime_modifier = crime_modifier - 4;
                quality_modifier = 3;
            }
        }

        trading_post.crime = Crime::random(crime_modifier);

        //Step 3: Points of Interest
        let number_of_shops = match par.number_shops {
            Some(val) => {
                if val >= 0 && val <= 9 {
                    val
                } else {
                    roll_num_shops(&trading_post.size)
                }
            }
            None => roll_num_shops(&trading_post.size),
        };

        for _ in 0..number_of_shops {
            trading_post.shops.push(Shop::random(quality_modifier));
        }

        let mut number_of_services = match par.number_services {
            Some(val) => {
                if val >= 0 && val <= 9 {
                    val
                } else {
                    roll_num_shops(&trading_post.size)
                }
            }
            None => roll_num_shops(&trading_post.size),
        };

        if trading_post.services.len() > 1 {
            number_of_services = number_of_services - 1;
        }

        for _ in 0..number_of_services {
            trading_post.services.push(Service::random());
        }

        if roll(2) == 2 {
            trading_post
                .places_of_worship
                .push(PlaceOfWorship::random())
        }

        //Extra Intrigue
        trading_post.recent_history = RecentHistory::random();
        trading_post.politics = Politics::random();
        trading_post.events = Event::random();
        trading_post.opportunity = Opportunities::random();
        trading_post.danger_level = DangerLevel::random();
        trading_post.danger_type = DangerType::random();

        match par.name {
            Some(val) => trading_post.name = val,
            None => trading_post.name = format!("{:?} Place", trading_post.environment),
        }

        trading_post
    }
}
