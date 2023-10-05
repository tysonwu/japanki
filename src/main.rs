use std::error::Error;
use std::str::FromStr;
// use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::de::{self, Deserializer, Unexpected};
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "Japanki")]
#[command(about = "Japanki")]
#[command(long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show vocabularies
    Show { category: Vec<String> },
    /// Do quiz
    Quiz { category: Vec<String> },
}

#[derive(Debug, Clone, Deserialize)]
enum Category {
    Unit,
    Atomic,
    Time,
    People,
    Places,
    Verb,
    Adjadv,
    Color,
    Direction,
    Nature,
    Food,
    Body,
    Home,
    Intangible,
    Activity,
    Wearables,
    ManMade,
    Stationery,
    Transport,
    Sentence,
}

impl std::str::FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unit" => Ok(Category::Unit),
            "atomic" => Ok(Category::Atomic),
            "time" => Ok(Category::Time),
            "people" => Ok(Category::People),
            "places" => Ok(Category::Places),
            "verb" => Ok(Category::Verb),
            "adjadv" => Ok(Category::Adjadv),
            "color" => Ok(Category::Color),
            "direction" => Ok(Category::Direction),
            "nature" => Ok(Category::Nature),
            "food" => Ok(Category::Food),
            "body" => Ok(Category::Body),
            "home" => Ok(Category::Home),
            "intangible" => Ok(Category::Intangible),
            "activity" => Ok(Category::Activity),
            "wearables" => Ok(Category::Wearables),
            "man made" => Ok(Category::ManMade),
            "stationery" => Ok(Category::Stationery),
            "transport" => Ok(Category::Transport),
            "sentence" => Ok(Category::Sentence),
            _ => Err(()),
        }
    }
}


fn from_str<'de, D>(deserializer: D) -> Result<Category, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Category::from_str(&s).map_err(|_| de::Error::invalid_value(Unexpected::Str(&s), &"a valid category"))
}


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Vocab {
    order: u16,
    hiragana: String,
    kanji: Option<String>,
    meaning: String,
    #[serde(deserialize_with = "from_str")]
    category: Category,
    example: Option<String>,
    romaji: String,
}

fn read_file() -> Result<Vec<Vocab>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/words.csv")?;
    let mut vocabs = Vec::new();
    for result in rdr.deserialize() {
        let record: Vocab = result?;
        vocabs.push(record);
    }
    Ok(vocabs)
}

fn main() {
    let cli = Cli::parse();

    println!("Reading in vocab database...");
    let _ = read_file();

    match &cli.command {
        Commands::Show { category } => {
            let mut cats = Vec::new();
            for cat in category{
                cats.push(Category::from_str(cat).unwrap());
            }
            println!("cmd {:?}", cats)
        }
        Commands::Quiz { category } => {
            let mut cats = Vec::new();
            for cat in category{
                cats.push(Category::from_str(cat).unwrap());
            }
            println!("cmd {:?}", cats)
        }
    }
}
