use std::fmt;

use strum_macros::{EnumIter, EnumString, Display};
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Vocab {
    pub order: u16,
    pub level: u8,
    pub hiragana: String,
    pub kanji: Option<String>,
    pub meaning: String,
    pub category: Category,
    pub example: Option<String>,
    pub romaji: String,
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
    pub fn short_display(&self){
        print!("{}", self.hiragana);

        let val = self.kanji.as_ref();
        if let Some(val) = val {
            print!(" | {}", val);
        }

        println!(" | {}", self.romaji);
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
