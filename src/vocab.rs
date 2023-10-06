use std::fmt;

use strum_macros::{EnumIter, EnumString, Display};
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Display, EnumIter, EnumString, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum Category {
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
    Manmade,
    Stationery,
    Transport,
    Sentence,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[allow(dead_code)]
pub struct Vocab {
    pub order: u16,
    hiragana: String,
    kanji: Option<String>,
    meaning: String,
    pub category: Category,
    example: Option<String>,
    romaji: String,
}

impl fmt::Display for Vocab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let default_empty = String::from("-");
        writeln!(f)?;
        write!(f, "[{}] | {} | ", self.category, self.order)?;
        write!(f, "{} | {} | {} | ", self.hiragana, self.kanji.as_ref().unwrap_or(&default_empty), self.romaji)?;
        write!(f, "{}", self.meaning)?;
        writeln!(f)?;
        Ok(())
    }
}
