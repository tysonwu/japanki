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

impl Category {
    pub const VARIANTS: &'static [Category] = &[
        Self::Unit,
        Self::Atomic,
        Self::Time,
        Self::People,
        Self::Places,
        Self::Verb,
        Self::Adjadv,
        Self::Color,
        Self::Direction,
        Self::Nature,
        Self::Food,
        Self::Body,
        Self::Home,
        Self::Intangible,
        Self::Activity,
        Self::Wearables,
        Self::Manmade,
        Self::Stationery,
        Self::Transport,
        Self::Sentence,
    ];
}
