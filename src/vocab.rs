use std::fmt;

use strum_macros::{EnumIter, EnumString, Display};
use serde::{Serialize, Deserialize};
use tabled::Tabled;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display, EnumIter, EnumString, Serialize, Deserialize)]
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

fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => String::from(s),
        None => String::from("-"),
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[derive(Tabled)]
pub struct Vocab {
    pub level: u8,
    pub order: u16,
    pub category: Category,
    pub hiragana: String,
    #[tabled(display_with = "display_option")]
    pub kanji: Option<String>,
    pub romaji: String,
    pub meaning: String,
    #[tabled(display_with = "display_option")]
    pub example: Option<String>,
}

impl fmt::Display for Vocab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "====== {} {} | Level {} =====", self.category, self.order, self.level)?;
        write!(f, "{}", self.hiragana)?;

        let val = self.kanji.as_ref();
        if let Some(val) = val {
            write!(f, " | {}", val)?;
        }

        write!(f, " | {}", self.romaji)?;
        writeln!(f, "\nMeaning: {}", self.meaning)?;

        let val = self.example.as_ref();
        if let Some(val) = val {
            writeln!(f, "Example: {}", val)?;
        }
        Ok(())
    }
}

impl Vocab {
    pub fn short_display(&self) {
        print!("{}", self.hiragana);
        let val = self.kanji.as_ref();
        if let Some(val) = val {
            print!(" | {}", val);
        }
        println!(" | {}", self.romaji);
    }

    pub fn line_display(&self) {
        println!(
            "{} {} | Level {} || {} | {} | {} | {} | {}",
            self.category,
            self.order,
            self.level,
            self.hiragana,
            self.kanji.as_ref().unwrap_or(&String::from(" - ")),
            self.romaji,
            self.meaning,
            self.example.as_ref().unwrap_or(&String::from(" - ")),
        )
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum MaskableVocabField {
    Hiragana,
    Romaji,
    Kanji,
    Meaning,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaskedVocab<'a, 'b> {
    pub vocab: &'a Vocab,
    pub masked_field: &'b Vec<MaskableVocabField>,
}

impl<'a, 'b> fmt::Display for MaskedVocab<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "====== {} {} | Level {} =====", self.vocab.category, self.vocab.order, self.vocab.level)?;

        if self.masked_field.contains(&MaskableVocabField::Hiragana) {
            write!(f, "Hiragana: ???")?;
        } else {
            write!(f, "{}", self.vocab.hiragana)?;
        }

        let val = self.vocab.kanji.as_ref();
        if let Some(val) = val {
            if self.masked_field.contains(&MaskableVocabField::Kanji) {
                write!(f, " | Kanji: ???")?;
            } else {
                write!(f, " | {}", val)?;
            }
        }

        if self.masked_field.contains(&MaskableVocabField::Romaji) {
            write!(f, " | Romaji: ???")?;
        } else {
            write!(f, " | {}", self.vocab.romaji)?;
        }

        if self.masked_field.contains(&MaskableVocabField::Meaning) {
            writeln!(f, "\nMeaning: ???")?;
        } else {
            writeln!(f, "\nMeaning: {}", self.vocab.meaning)?;
        }
        Ok(())
    }
}
