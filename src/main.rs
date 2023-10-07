pub mod vocab;
pub mod config;

use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::process;

use rand::seq::SliceRandom;
use clap::Parser;
use strum::IntoEnumIterator;
use inquire::{
    MultiSelect, Confirm, Text, list_option::ListOption, validator::Validation
};
use dirs::home_dir;
use serde_yaml::Error;

use vocab::{
    Vocab, Category, MaskableVocabField, MaskedVocab
};
use config::Progress;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(name = "Japanki")]
#[command(about = "Learn Japanese N5 vocabularies 🇯🇵")]
#[command(long_about = None)]
#[command(next_line_help = true)]
enum Cli {
    /// Show vocabularies
    Show {
        #[clap(subcommand)]
        subcmd: ShowSubCommand,
    },
    /// Do quiz
    Quiz {
        #[clap(subcommand)]
        subcmd: QuizSubCommand,
    },
    /// Show current study progress, level up, or reset
    Progress {
        #[clap(subcommand)]
        subcmd: ProgressSubCommand,
    },
    /// List out all categories
    List,
}

#[derive(Parser, Debug)]
enum ShowSubCommand {
    All {
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
    },
    Some {
        category: Vec<String>,
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
    },
}

#[derive(Parser, Debug)]
enum QuizSubCommand {
    All {
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
        #[clap(long)]
        meaning: bool,
    },
    Some {
        category: Vec<String>,
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
        #[clap(long)]
        meaning: bool,
    },
}

#[derive(Parser, Debug)]
enum ProgressSubCommand {
    Now,
    Up,
    Reset,
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
            Ok(false) => {
                println!("Exit. さよなら！");
                break
            }
            _ => {
                println!("Exit. さよなら！");
                process::exit(1);
            },
        }
    }
}

fn clean_romaji(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn quiz_random_loop(vocabs: Vec<Vocab>, masked_fields: Vec<MaskableVocabField>, answer_field: MaskableVocabField) {
    // quiz a vocab randomly one time every loop
    let formatter = &|ans| match ans {
        true => "▷".to_owned(),
        false => "Exit".to_owned(),
    };
    loop {
        let unmasked_vocab = vocabs.choose(&mut rand::thread_rng()).unwrap();
        let choice = MaskedVocab { vocab: unmasked_vocab, masked_field: &masked_fields };
        println!("{}", choice);

        match answer_field {
            MaskableVocabField::Meaning => {
                let ans = Text::new("Meaning is: ").prompt();
                match ans {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Exit. さよなら！");
                        process::exit(1);
                    },
                }
                println!("🔎 Answer is {}", &unmasked_vocab.meaning);
            },
            MaskableVocabField::Romaji => {
                let ans = Text::new("Romaji is: ").prompt();
                match ans {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Exit. さよなら！");
                        process::exit(1);
                    },
                }
                let model_ans = clean_romaji(&unmasked_vocab.romaji);
                if model_ans == ans.unwrap() {
                    println!("✅ Correct! It is {}", &unmasked_vocab.romaji);
                } else {
                    println!("❌ Oops! It should be {}", &unmasked_vocab.romaji);
                }
            },
            _ => {
                // unhandled maskable vocab field
                println!("Some error. Exit.");
                process::exit(1);
            }
        }

        let next = Confirm::new("Next?")
            .with_default(true)
            .with_formatter(formatter)
            .with_help_message("Press [Enter] to continue")
            .prompt();

        match next {
            Ok(true) => {},
            Ok(false) => {
                println!("Exit. さよなら！");
                break
            }
            _ => {
                println!("Some error. Exit.");
                process::exit(1);
            },
        }
    }
}

fn quiz_random_meaning(vocabs: Vec<Vocab>) {
    let masked_fields: Vec<MaskableVocabField> = vec![MaskableVocabField::Kanji, MaskableVocabField::Meaning];
    let answer_field: MaskableVocabField = MaskableVocabField::Meaning;
    quiz_random_loop(vocabs, masked_fields, answer_field);
}

fn quiz_random_romaji(vocabs: Vec<Vocab>) {
    let masked_fields: Vec<MaskableVocabField> = vec![MaskableVocabField::Hiragana, MaskableVocabField::Romaji];
    let answer_field: MaskableVocabField = MaskableVocabField::Romaji;
    quiz_random_loop(vocabs, masked_fields, answer_field);
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

fn filter_db(db: Vec<Vocab>, cats: Vec<Category>, current_progress: &Progress, kanji: &bool) -> Vec<Vocab> {
    let filtered_vocabs: Vec<Vocab> = db
        .into_iter()
        .filter(|word| cats.contains(&word.category))
        .filter(|word| -> bool {
            if *kanji {
                word.kanji.is_some()
            } else {
                true
            }
        })
        .filter(|word| -> bool {
            word.level <= current_progress.level
        })
        .collect();
    filtered_vocabs
}

fn read_progress(file_name: &str) -> Result<Progress, Error> {
    let home = home_dir().expect("Could not get home directory");
    let file_path = home.join(Path::new(file_name));
    let content = fs::read_to_string(file_path).expect("Could not read file");
    let config: Progress = serde_yaml::from_str(&content)?;
    Ok(config)
}

fn write_progress(progress: Progress, file_name: &str) {
    let home = home_dir().expect("Could not get home directory");
    let file_path = home.join(Path::new(file_name));
    let content = serde_yaml::to_string(&progress).expect("Could not serialize data");
    fs::write(file_path, content).expect("Could not write file");
}

fn read_file() -> Vec<Vocab> {
    println!("Reading in vocab database...");
    let mut rdr = csv::Reader::from_path("data/words.csv").unwrap();
    let mut vocabs = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(vocab) => {vocabs.push(vocab)},
            Err(_) => {
                println!("Error reading file. Exit.");
                process::exit(1);
            }
        }
    }
    vocabs
}

fn main() {
    let cli = Cli::parse();

    // read progress from dotfile at home dir
    let mut current_progress: Progress = read_progress("./.japanki/progress.yaml").expect("Could not parse progress file.");

    // let mut current_progress: HashMap<String, u16> = HashMap::new();
    // current_progress.insert(String::from("max"), 100);

    match &cli {
        Cli::Show { subcmd } => {
            let db = read_file();
            match subcmd {
                ShowSubCommand::All { no_progress, kanji } => {
                    if no_progress.to_owned() { current_progress = Progress{ level: 10 } };
                    let cats: Vec<Category> = Category::iter().collect();
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji);
                    show_random_loop(filtered_vocabs)
                },
                ShowSubCommand::Some { category, no_progress, kanji } => {
                    if no_progress.to_owned() { current_progress = Progress{ level: 10 } };
                    let cats = process_cats_input(category);
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji);
                    show_random_loop(filtered_vocabs);
                },
            }
        },
        Cli::Quiz { subcmd } => {
            let db = read_file();
            match subcmd {
                QuizSubCommand::All { no_progress, kanji, meaning } => {
                    if no_progress.to_owned() { current_progress = Progress{ level: 10 } };
                    let cats: Vec<Category> = Category::iter().collect();
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji);
                    if meaning.to_owned() {
                        quiz_random_meaning(filtered_vocabs);
                    } else {
                        quiz_random_romaji(filtered_vocabs);
                    }
                },
                QuizSubCommand::Some { category, no_progress, kanji, meaning } => {
                    if no_progress.to_owned() { current_progress = Progress{ level: 10 } };
                    let cats = process_cats_input(category);
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji);
                    if meaning.to_owned() {
                        quiz_random_meaning(filtered_vocabs);
                    } else {
                        quiz_random_romaji(filtered_vocabs);
                    }
                }
            }
        },
        Cli::Progress { subcmd } => {
            match subcmd {
                ProgressSubCommand::Now => {
                    println!("Current progress: {} out of 10", &current_progress.level);
                },
                ProgressSubCommand::Up => {
                    let new_prog = current_progress.level + 1;
                    if new_prog > 10 {
                        println!("You have achieved the highest level already!")
                    } else {
                        write_progress(Progress { level: new_prog }, "./.japanki/progress.yaml");
                        println!("Level up progress to {}.", new_prog);
                    }
                },
                ProgressSubCommand::Reset => {
                    write_progress(Progress { level: 1 }, "./.japanki/progress.yaml");
                    println!("Reset progress to 1.");
                },
            }
        },
        Cli::List {} => {
            println!("Available categories:");
            for cat in Category::iter() {
                println!(" - {:}", cat)
            }
        }
    }
}
