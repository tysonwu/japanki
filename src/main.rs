pub mod vocab;

use std::str::FromStr;
use std::error::Error;

use rand::seq::SliceRandom;
use clap::{Parser, Subcommand};

use strum::IntoEnumIterator;
use inquire::{
    MultiSelect, Confirm, list_option::ListOption, validator::Validation
};

use vocab::{Vocab, Category};

#[derive(Parser)]
#[command(author, version)]
#[command(name = "Japanki")]
#[command(about = "Learn Japanese N5 vocabularies 🇯🇵")]
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


fn read_file() -> Result<Vec<Vocab>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/words.csv")?;
    let mut vocabs = Vec::new();
    for result in rdr.deserialize() {
        let record: Vocab = result?;
        vocabs.push(record);
    }
    Ok(vocabs)
}

fn show_random_loop(vocabs: Vec<Vocab>) {
    // show a vocab randomly one time every loop
    loop {
        println!("{}", vocabs.choose(&mut rand::thread_rng()).unwrap());
        let next = Confirm::new("Next")
            .with_default(true)
            .with_help_message("Press [Enter] to continue")
            .prompt();

        match next {
            Ok(true) => {},
            _ => {
                break
            },
        }
    }
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
                cats = MultiSelect::new("Select categories:", Category::iter().collect())
                    .with_validator(validator)
                    .with_page_size(100)
                    .prompt()
                    .unwrap();
            } else {
                cats = Vec::new();
                for cat in category{
                    cats.push(Category::from_str(cat).unwrap());
                }
            }
            println!("Selected: {:?}", cats);

            // read file
            println!("Reading in vocab database...");
            let db = read_file();
            match db {
                Ok(vocabs) => {
                    let filtered_vocabs: Vec<Vocab> = vocabs
                        .into_iter()
                        .filter(|word| cats.contains(&word.category))
                        .collect();
                    // println!("{}", filtered_vocabs.len());
                    // for entry in filtered_vocabs {
                        // println!("{:}", entry)
                    // }
                    show_random_loop(filtered_vocabs);
                },
                Err(_) => panic!("Read file error!"),
            }
        }
        Commands::Quiz { category } => {
            println!("{:?}, Not implemented yet :(", category)
        }
    }
}
