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
pub struct Vocab {
    pub order: u16,
    pub hiragana: String,
    pub kanji: Option<String>,
    meaning: String,
    pub category: Category,
    example: Option<String>,
    pub romaji: String,
}

impl fmt::Display for Vocab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "====== {} {} =====", self.category, self.order)?;
        write!(f, "{}", self.hiragana)?;

        let val = self.kanji.as_ref();
        if let Some(val) = val {
            write!(f, " | {}", val)?;
        }

        write!(f, " | {}", self.romaji)?;
        write!(f, "\nMeaning: {}", self.meaning)?;

        let val = self.example.as_ref();
        if let Some(val) = val {
            write!(f, "\nExample: {}", val)?;
        }
        writeln!(f)?;
        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum MaskableVocabField {
    Hiragana,
    Kanji,
    Romaji,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaskedVocab<'a> {
    pub vocab: &'a Vocab,
    pub masked_field: Vec<MaskableVocabField>,
}

impl<'a> fmt::Display for MaskedVocab<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "====== {} {} =====", self.vocab.category, self.vocab.order)?;

        if self.masked_field.contains(&MaskableVocabField::Hiragana) {
            write!(f, "?????")?;
        } else {
            write!(f, "{}", self.vocab.hiragana)?;
        }

        let val = self.vocab.kanji.as_ref();
        if let Some(val) = val {
            if self.masked_field.contains(&MaskableVocabField::Kanji) {
                write!(f, " | ?????")?;
            } else {
                write!(f, " | {}", val)?;
            }
        }

        if self.masked_field.contains(&MaskableVocabField::Romaji) {
            write!(f, " | ?????")?;
        } else {
            write!(f, " | {}", self.vocab.romaji)?;
        }

        writeln!(f)?;
        Ok(())
    }
}
