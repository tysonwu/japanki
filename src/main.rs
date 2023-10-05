pub mod category;

use std::str::FromStr;
use std::error::Error;
use std::fmt;

use rand::seq::SliceRandom;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use inquire::{
    MultiSelect, list_option::ListOption, validator::Validation
};

use category::Category;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "Japanki")]
#[command(about = "Learn Japanese N5 vocabularies 👹")]
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

#[derive(Debug, Deserialize, PartialEq)]
#[allow(dead_code)]
struct Vocab {
    order: u16,
    hiragana: String,
    kanji: Option<String>,
    meaning: String,
    category: Category,
    example: Option<String>,
    romaji: String,
}

impl fmt::Display for Vocab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] | {} | ", self.category, self.order)?;
        write!(f, "{} | {:?} | {}", self.hiragana, self.kanji, self.romaji)?;
        Ok(())
    }
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

fn start_showing(cat: Vec<Category>) {
    println!("Reading in vocab database...");
    let vocabs = read_file();
    match vocabs {
        Ok(vocabs) => {
            let filtered_vocabs: Vec<Vocab> = vocabs
                .into_iter()
                .filter(|word| cat.contains(&word.category))
                .collect();
            // println!("{}", filtered_vocabs.len());
            // for entry in filtered_vocabs {
                // println!("{:}", entry)
            // }

            // show a vocab randomly
            println!("{}", filtered_vocabs.choose(&mut rand::thread_rng()).unwrap());
        },
        Err(_) => panic!("Read file error!"),
    };
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Show { category } => {
            let mut cats;
            if category.last().is_none() {
                let validator = |a: &[ListOption<&Category>]| {
                    match a.is_empty() {
                        true => Ok(Validation::Invalid("Please select at least one category.".into())),
                        false => Ok(Validation::Valid),
                    }
                };
                cats = MultiSelect::new("Select categories:", Category::VARIANTS.to_vec())
                    .with_validator(validator)
                    .prompt()
                    .unwrap();
            } else {
                cats = Vec::new();
                for cat in category{
                    cats.push(Category::from_str(cat).unwrap());
                }
            }
            println!("Selected: {:?}", cats);
            start_showing(cats);
        }
        Commands::Quiz { category } => {
            println!("{:?}, Not implemented yet :(", category)
        }
    }
}
