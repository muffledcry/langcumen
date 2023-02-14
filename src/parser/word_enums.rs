#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Degree {
    Positive,
    Comparative,
    Superlative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifies {
    Verb,
    Adjective,
    Adverb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Before,
    After
}

pub enum PrepositionCase {
    Nominative,
    Accusative,
    Dative,
    Genitive,
}