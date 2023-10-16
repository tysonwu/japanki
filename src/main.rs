pub mod vocab;
pub mod config;

use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::process;
use std::collections::HashMap;

use rand::seq::SliceRandom;
use clap::Parser;
use strum::IntoEnumIterator;
use inquire::{
    MultiSelect, Confirm, Text, list_option::ListOption, validator::Validation
};
use dirs::home_dir;
use serde_yaml::Error;
use tabled::Table;
use tabled::settings::{Style, Width, Modify, object::Columns};

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
    /// Display
    Display {
        #[clap(subcommand)]
        subcmd: DisplaySubCommand
    }
}

#[derive(Parser, Debug)]
enum ShowSubCommand {
    /// Show all categories
    All {
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
    },
    /// Show selected categories
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
    /// Quiz all categories
    All {
        #[clap(long)]
        no_progress: bool,
        #[clap(long)]
        kanji: bool,
        #[clap(long)]
        meaning: bool,
    },
    /// Quiz some categories
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
    /// Show current level for each categories
    Now,
    /// Level up categories
    Up,
    /// Level down categories
    Down,
    /// Reset all categories to lowest level
    Reset,
}

#[derive(Parser, Debug)]
enum DisplaySubCommand {
    /// Display all vocabs in current progress
    All {
        #[clap(long)]
        kanji: bool,
        #[clap(long)]
        no_progress: bool,
    },
    /// Display all vocabs in current progress and selected categories
    Some {
        category: Vec<String>,
        #[clap(long)]
        kanji: bool,
        #[clap(long)]
        no_progress: bool,
    },
}

fn show_random_loop(vocabs: Vec<Vocab>) {
    // show a vocab randomly one time every loop
    println!("{} possible vocabs", vocabs.len());
    loop {
        let choice = vocabs.choose(&mut rand::thread_rng()).unwrap();
        println!("{}", choice);

        let next = confirmation_prompt("Continue");
        match next {
            true => {},
            false => {
                println!("Exit. さよなら！");
                break
            }
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
    println!("{} possible vocabs", vocabs.len());
    loop {
        let unmasked_vocab = vocabs.choose(&mut rand::thread_rng()).unwrap();
        let choice = MaskedVocab { vocab: unmasked_vocab, masked_field: &masked_fields };
        println!("{}", choice);

        match answer_field {
            MaskableVocabField::Meaning => {
                let ans = Text::new("Meaning is:").prompt();
                match ans {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Exit. さよなら！");
                        process::exit(1);
                    },
                }
                println!("🔎 Answer | {}", &unmasked_vocab.meaning);
                println!("\n");
            },
            MaskableVocabField::Romaji => {
                let ans = Text::new("Romaji is:").prompt();
                match ans {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Exit. さよなら！");
                        process::exit(1);
                    },
                }
                let model_ans = clean_romaji(&unmasked_vocab.romaji);
                if model_ans == ans.unwrap() {
                    print!("✅ Correct! It is ");
                    unmasked_vocab.short_display();
                } else {
                    print!("❌ Oops! It should be ");
                    unmasked_vocab.short_display();
                    println!();
                    let corr = Text::new("✏️ Correction:").prompt();
                    match corr {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Exit. さよなら！");
                            process::exit(1);
                        },
                    }
                }
                println!();
            },
            _ => { unreachable!() },
        }

        let next = confirmation_prompt("Next question");
        match next {
            true => {},
            false => {
                println!("Exit. さよなら！");
                break
            }
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
        cats = multiselect_prompt("Select categories:");
    } else {
        cats = Vec::new();
        for cat in category{
            let c = Category::from_str(cat);
            match c {
                Ok(v) => cats.push(v),
                Err(_) => {
                    eprintln!("Bad category input!");
                    eprintln!("Available categories:");
                    for c in Category::iter(){
                        eprintln!("- {}", c);
                    }
                    process::exit(1);
                }
            }
        }
    }
    cats
}

fn multiselect_prompt(display: &str) -> Vec<Category> {
    let validator = |a: &[ListOption<&Category>]| {
        match a.is_empty() {
            true => Ok(Validation::Invalid("Please select at least one category.".into())),
            false => Ok(Validation::Valid),
        }
    };
    let cats = MultiSelect::new(display, Category::iter().collect())
        .with_page_size(Category::iter().len()) // always show all at once
        .with_validator(validator)
        .prompt();
    match cats {
        Ok(v) => { v },
        Err(_) => { process::exit(1) },
    }
}

fn confirmation_prompt(display: &str) -> bool {
    let formatter = &|ans| match ans {
        true => "▷".to_owned(),
        false => "Exit".to_owned(),
    };
    let confirmation = Confirm::new(display)
        .with_default(true)
        .with_formatter(formatter)
        .with_help_message("Press [Enter] to reset")
        .prompt();
    match confirmation {
        Ok(v) => { v },
        Err(_) => { process::exit(1) },
    }
}

fn filter_db(db: Vec<Vocab>, cats: Vec<Category>, current_progress: &HashMap<Category, Progress>, kanji: &bool, ignore_progress: &bool) -> Vec<Vocab> {
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
            if *ignore_progress {
                true
            } else {
                word.level <= current_progress.get(&word.category).unwrap().level
            }
        })
        .collect();
    filtered_vocabs
}

fn read_progress(file_name: &str) -> Result<HashMap<Category, Progress>, Error> {
    let home = home_dir().expect("Could not get home directory");
    let file_path = home.join(Path::new(file_name));
    let content = fs::read_to_string(file_path).expect("Could not read file");
    let config: HashMap<Category, Progress> = serde_yaml::from_str(&content)?;
    Ok(config)
}

fn write_progress(progress: &HashMap<Category, Progress>, file_name: &str) {
    let home = home_dir().expect("Could not get home directory");
    let file_path = home.join(Path::new(file_name));
    let content = serde_yaml::to_string(progress).expect("Could not serialize data");
    fs::write(file_path, content).expect("Could not write file");
}

fn read_file(file_name: &str) -> Vec<Vocab> {
    println!("Reading in vocab database...");
    let home = home_dir().expect("Could not get home directory");
    let file_path = home.join(Path::new(file_name));
    let mut rdr = csv::Reader::from_path(file_path).unwrap();
    let mut vocabs = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Ok(vocab) => {vocabs.push(vocab)},
            Err(_) => {
                eprintln!("Error reading file. Exit.");
                process::exit(1);
            }
        }
    }
    vocabs
}

fn display_progress(current_progress: &HashMap<Category, Progress>) {
    println!("=== Current progress ===");
    for cat in Category::iter() {
        println!(
            "{:<12}  Level {:>2} / {}",
            cat,
            current_progress.get(&cat).unwrap().level,
            current_progress.get(&cat).unwrap().max_level
        );
    }
}

fn line_display_vocabs(vocabs: Vec<Vocab>) {
    let mut table = Table::new(vocabs);
    table.with(Style::rounded());
    table.with(Modify::new(Columns::new(1..)).with(Width::wrap(40).keep_words()));
    println!("{}", table);
}

fn main() {
    let cli = Cli::parse();
    let db_path: &str = "./.japanki/words.csv";
    let config_path: &str = "./.japanki/progress.yaml";

    // read progress from dotfile at home dir
    let mut current_progress: HashMap<Category, Progress> = read_progress(config_path).expect("Could not parse progress file.");

    match &cli {
        Cli::Show { subcmd } => {
            let db = read_file(db_path);
            match subcmd {
                ShowSubCommand::All { no_progress, kanji } => {
                    let cats: Vec<Category> = Category::iter().collect();
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
                    show_random_loop(filtered_vocabs)
                },
                ShowSubCommand::Some { category, no_progress, kanji } => {
                    let cats = process_cats_input(category);
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
                    show_random_loop(filtered_vocabs);
                },
            }
        },
        Cli::Quiz { subcmd } => {
            let db = read_file(db_path);
            match subcmd {
                QuizSubCommand::All { no_progress, kanji, meaning } => {
                    let cats: Vec<Category> = Category::iter().collect();
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
                    if meaning.to_owned() {
                        quiz_random_meaning(filtered_vocabs);
                    } else {
                        quiz_random_romaji(filtered_vocabs);
                    }
                },
                QuizSubCommand::Some { category, no_progress, kanji, meaning } => {
                    let cats = process_cats_input(category);
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
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
                    display_progress(&current_progress)
                },
                ProgressSubCommand::Up => {
                    let cats = multiselect_prompt("Select categories to level up");
                    for cat in cats {
                        current_progress.get_mut(&cat).unwrap().up();
                    }
                    display_progress(&current_progress);
                    write_progress(&current_progress, config_path);
                },
                ProgressSubCommand::Down => {
                    let cats = multiselect_prompt("Select categories to level down");
                    for cat in cats {
                        current_progress.get_mut(&cat).unwrap().down();
                    }
                    display_progress(&current_progress);
                    write_progress(&current_progress, config_path);
                },
                ProgressSubCommand::Reset => {
                    let confirmation = confirmation_prompt("Are you sure to reset progress? Progress will be lost.");
                    match confirmation {
                        true => {
                            for cat in Category::iter() {
                                current_progress.get_mut(&cat).unwrap().reset();
                            }
                            display_progress(&current_progress);
                            write_progress(&current_progress, config_path);
                        },
                        false => {
                            println!("Exit. さよなら！");
                        },
                    }
                },
            }
        },
        Cli::List {} => {
            println!("Available categories:");
            for cat in Category::iter() {
                println!(" - {:}", cat)
            }
        },
        Cli::Display { subcmd } => {
            let db = read_file(db_path);
            match subcmd {
                DisplaySubCommand::All { kanji, no_progress } => {
                    let cats: Vec<Category> = Category::iter().collect();
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
                    line_display_vocabs(filtered_vocabs);
                },
                DisplaySubCommand::Some { category, kanji, no_progress } => {
                    let cats = process_cats_input(category);
                    let filtered_vocabs: Vec<Vocab> = filter_db(db, cats, &current_progress, kanji, no_progress);
                    line_display_vocabs(filtered_vocabs);
                },
            }
        }
    }
}
