pub mod vocab;

use std::str::FromStr;
use std::error::Error;
use std::process;

use rand::seq::SliceRandom;
use clap::{Parser, Subcommand};

use strum::IntoEnumIterator;
use inquire::{
    MultiSelect, Confirm, Text, list_option::ListOption, validator::Validation
};

use vocab::{
    Vocab, Category, MaskableVocabField, MaskedVocab
};

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
    println!("Reading in vocab database...");
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
    let formatter = &|ans| match ans {
        true => "▷".to_owned(),
        false => "Exit".to_owned(),
    };
    loop {
        let choice = vocabs.choose(&mut rand::thread_rng()).unwrap();
        println!("{}", choice);
        let next = Confirm::new("Show next")
            .with_default(true)
            .with_formatter(formatter)
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

fn quiz_random_loop(vocabs: Vec<Vocab>) {
    // quiz a vocab randomly one time every loop
    loop {
        let masked_fields = vec![MaskableVocabField::Hiragana];
        let unmasked_vocab = vocabs.choose(&mut rand::thread_rng()).unwrap();
        let choice = MaskedVocab { vocab: unmasked_vocab, masked_field: masked_fields };
        println!("{}", choice);
        let ans = Text::new("Hiragana:")
            .with_help_message("Your answer:")
            .prompt();

        match ans {
            Ok(ans) => {
                let model_ans = &unmasked_vocab.hiragana;
                println!("Your answer: {}", ans);
                println!("Model answer: {}", model_ans);
                if model_ans == &ans {
                    println!("correct")
                } else {
                    println!("wrong")
                }
                break
            },
            Err(_) => println!("An error Occured."),
        }
    }
}

fn process_cats_input(category: &Vec<String>) -> Vec<Category> {
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
            let c = Category::from_str(cat);
            match c {
                Ok(v) => cats.push(v),
                Err(_) => {
                    println!("Bad category input!");
                    println!("Available categories:");
                    for c in Category::iter(){
                        println!("- {}", c);
                    }
                    process::exit(1);
                }
            }
        }
    }
    println!("Selected: {:?}", cats);
    cats
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Show { category } => {
            let cats = process_cats_input(category);

            // read file
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
            let cats = process_cats_input(category);

            // read file
            let db = read_file();
            match db {
                Ok(vocabs) => {
                    let filtered_vocabs: Vec<Vocab> = vocabs
                        .into_iter()
                        .filter(|word| cats.contains(&word.category))
                        .collect();
                    quiz_random_loop(filtered_vocabs);
                },
                Err(_) => panic!("Read file error!"),
            }
        }
    }
}
