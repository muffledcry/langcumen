use std::fmt::Display;
use super::word_enums::{Gender, Number, Degree, Modifies, Position, PrepositionCase, ConjunctionType, ConjunctionCategory, Sentiment};


pub enum PartOfSpeech {
    Noun(Word),
    Pronoun(Word),
    Verb(Word),
    Adjective(Word),
    Adverb(Word),
    Preposition(Word),
    Conjunction(Word),
    Interjection(Word),
    Article(Word),
}

pub struct Word {
    base_form: String,
}

impl Word {
    fn new(base_form: String) -> Word {
        Word {
            base_form: base_form,
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word") // <-- This needs to be changed to display the word
    }
}

pub trait Noun {
    fn new(singular: &str) -> Self;
    fn pluralize(&self) -> String;
    fn singularize(&self) -> String;
}

impl Noun for Word {
    fn new(singular: &str) -> Word {
        Word {
            base_form: singular.to_string(),
        }
    }

    fn pluralize(&self) -> String {
        format!("{}s", self)
    }

    fn singularize(&self) -> String {
        self.to_string()
    }
}

impl Noun for PartOfSpeech {
    fn new(singular: &str) -> PartOfSpeech {
        PartOfSpeech::Noun(Word::new(singular.to_string()))
    }

    fn pluralize(&self) -> String {
        match self {
            PartOfSpeech::Noun(word) => word.pluralize(),
            _ => "".to_string(),
            // other cases
        }
    }

    fn singularize(&self) -> String {
        match self {
            PartOfSpeech::Noun(word) => word.singularize(),
            _ => "".to_string(),
        }
    }
}


pub trait Pronoun {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, gender: Gender, number: Number) -> Self;
    fn subject_form(&self) -> String;
    fn object_form(&self) -> String;
    fn possessive_form(&self) -> String;
    fn reflexive_form(&self) -> String;
    fn pronoun_gender(&self) -> Gender;
    fn pronoun_number(&self) -> Number;
}

impl Pronoun for Word {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, pronoun_gender: Gender, number: Number) -> Word {
        Word {
            base_form: subject_form.to_string(),
        }
    }

    fn subject_form(&self) -> String {
        self.to_string()
    }

    fn object_form(&self) -> String {
        self.to_string()
    }

    fn possessive_form(&self) -> String {
        self.to_string()
    }

    fn reflexive_form(&self) -> String {
        self.to_string()
    }

    fn pronoun_gender(&self) -> Gender {
        Gender::Neutral
    }

    fn pronoun_number(&self) -> Number {
        Number::Singular
    }
}

impl Pronoun for PartOfSpeech {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, gender: Gender, number: Number) -> PartOfSpeech {
        PartOfSpeech::Pronoun(Word::new(subject_form.to_string()))
    }

    fn subject_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.subject_form(),
            _ => "".to_string(),
        }
    }

    fn object_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.object_form(),
            _ => "".to_string(),
        }
    }

    fn possessive_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.possessive_form(),
            _ => "".to_string(),
        }
    }

    fn reflexive_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.reflexive_form(),
            _ => "".to_string(),
        }
    }

    fn pronoun_gender(&self) -> Gender {
        match self {
            PartOfSpeech::Pronoun(word) => word.pronoun_gender(),
            _ => Gender::Neutral,
        }
    }

    fn pronoun_number(&self) -> Number {
        match self {
            PartOfSpeech::Pronoun(word) => word.pronoun_number(),
            _ => Number::Singular,
            // other cases
        }
    }
}


pub trait Verb {
    fn new(base_form: &str) -> Self;
    fn conjugate(&self, tense: &str) -> String;
    fn verb_base_form(&self) -> String;
    fn past_tense(&self) -> String;
    fn past_participle(&self) -> String;
    fn present_tense(&self) -> String;
    fn present_participle(&self) -> String;
    fn third_person_singular(&self) -> String;
    fn present_tense_singular(&self) -> String;
    fn present_tense_plural(&self) -> String;
    fn infinitive(&self) -> String;
}

impl Verb for Word {
    fn new(base_form: &str) -> Word {
        Word {
            base_form: base_form.to_string(),
        }
    }

    fn conjugate(&self, tense: &str) -> String {
        match tense {
            "past" => self.past_tense(),
            "past_participle" => self.past_participle(),
            "present" => self.present_tense(),
            "present_participle" => self.present_participle(),
            "third_person_singular" => self.third_person_singular(),
            "present_tense_singular" => self.present_tense_singular(),
            "present_tense_plural" => self.present_tense_plural(),
            "infinitive" => self.infinitive(),
            _ => self.verb_base_form(),
        }
    }

    fn verb_base_form(&self) -> String {
        self.to_string()
    }

    fn past_tense(&self) -> String {
        format!("{}ed", self)
    }

    fn past_participle(&self) -> String {
        format!("{}ed", self)
    }

    fn present_tense(&self) -> String {
        format!("{}s", self)
    }

    fn present_participle(&self) -> String {
        format!("{}ing", self)
    }

    fn third_person_singular(&self) -> String {
        format!("{}s", self)
    }

    fn present_tense_singular(&self) -> String {
        format!("{}s", self)
    }

    fn present_tense_plural(&self) -> String {
        self.to_string()
    }

    fn infinitive(&self) -> String {
        format!("to {}", self)
    }
}

impl Verb for PartOfSpeech {
    fn new(base_form: &str) -> PartOfSpeech {
        PartOfSpeech::Verb(Word::new(base_form.to_string()))
    }

    fn conjugate(&self, tense: &str) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.conjugate(tense),
            _ => "".to_string(),
        }
    }

    fn verb_base_form(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.verb_base_form(),
            _ => "".to_string(),
        }
    }

    fn past_tense(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.past_tense(),
            _ => "".to_string(),
        }
    }

    fn past_participle(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.past_participle(),
            _ => "".to_string(),
        }
    }

    fn present_tense(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense(),
            _ => "".to_string(),
        }
    }

    fn present_participle(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_participle(),
            _ => "".to_string(),
        }
    }

    fn third_person_singular(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.third_person_singular(),
            _ => "".to_string(),
        }
    }

    fn present_tense_singular(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense_singular(),
            _ => "".to_string(),
        }
    }

    fn present_tense_plural(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense_plural(),
            _ => "".to_string(),
        }
    }

    fn infinitive(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.infinitive(),
            _ => "".to_string(),
        }
    }
}


pub trait Adjective {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> Self;
    fn adjust_degree(&self, degree: Degree) -> Self;
    fn adjective_base_form(&self) -> String;
    fn adjective_gender(&self) -> Gender;
    fn adjective_number(&self) -> Number;
    fn adjective_degree(&self) -> Degree;
    fn adjective_position(&self) -> Position;
}

impl Adjective for Word {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> Word {
        Word {
            base_form: base_form.to_string(),
        }
    }

    fn adjust_degree(&self, degree: Degree) -> Word {
        Word {
            base_form: self.adjective_base_form(),
        }
    }

    fn adjective_base_form(&self) -> String {
        self.to_string()
    }

    fn adjective_gender(&self) -> Gender {
        Gender::Neutral
    }

    fn adjective_number(&self) -> Number {
        Number::Singular
    }

    fn adjective_degree(&self) -> Degree {
        Degree::Positive
    }

    fn adjective_position(&self) -> Position {
        Position::Before
    }
}

impl Adjective for PartOfSpeech {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> PartOfSpeech {
        PartOfSpeech::Adjective(Word::new(base_form.to_string()))
    }

    fn adjust_degree(&self, degree: Degree) -> PartOfSpeech {
        match self {
            PartOfSpeech::Adjective(word) => PartOfSpeech::Adjective(word.adjust_degree(degree)),
            _ => PartOfSpeech::Adjective(Word::new("".to_string())),  
        }
    }

    fn adjective_base_form(&self) -> String {
        match self {
            PartOfSpeech::Adjective(word) => word.base_form(),
            _ => "".to_string(),
        }
    }

    fn adjective_gender(&self) -> Gender {
        match self {
            PartOfSpeech::Adjective(word) => word.adjective_gender(),
            _ => Gender::Neutral,
        }
    }

    fn adjective_number(&self) -> Number {
        match self {
            PartOfSpeech::Adjective(word) => word.adjective_number(),
            _ => Number::Singular,
        }
    }

    fn adjective_degree(&self) -> Degree {
        match self {
            PartOfSpeech::Adjective(word) => word.adjective_degree(),
            _ => Degree::Positive,
        }
    }

    fn adjective_position(&self) -> Position {
        match self {
            PartOfSpeech::Adjective(word) => word.adjective_position(),
            _ => Position::Before,
        }
    }
}


pub trait Adverb {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> Self;
    fn adverb_base_form(&self) -> String;
    fn modifies(&self) -> Modifies;
    fn adverb_position(&self) -> Position;
}

impl Adverb for Word {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> Word {
        Word {
            base_form: base_form.to_string(),
        }
    }

    fn adverb_base_form(&self) -> String {
        self.to_string()
    }

    fn modifies(&self) -> Modifies {
        Modifies::Verb
    }

    fn adverb_position(&self) -> Position {
        Position::After
    }
}

impl Adverb for PartOfSpeech {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> PartOfSpeech {
        PartOfSpeech::Adverb(Word::new(base_form.to_string()))
    }

    fn adverb_base_form(&self) -> String {
        match self {
            PartOfSpeech::Adverb(word) => word.base_form(),
            _ => "".to_string(),
        }
    }

    fn modifies(&self) -> Modifies {
        match self {
            PartOfSpeech::Adverb(word) => word.modifies(),
            _ => Modifies::Verb,
        }
    }

    fn adverb_position(&self) -> Position {
        match self {
            PartOfSpeech::Adverb(word) => word.adverb_position(),
            _ => Position::After,
        }
    }
}


pub trait Preposition {
    fn new(base_form: &str, case: PrepositionCase) -> Self;
    fn preposition_base_form(&self) -> String;
    fn case(&self) -> PrepositionCase;
}

impl Preposition for Word {
    fn new(base_form: &str, case: PrepositionCase) -> Word {
        Word {
            base_form: base_form.to_string(),
        }
    }

    fn preposition_base_form(&self) -> String {
        self.to_string()
    }

    fn case(&self) -> PrepositionCase {
        PrepositionCase::Accusative
    }
}

impl Preposition for PartOfSpeech {
    fn new(base_form: &str, case: PrepositionCase) -> PartOfSpeech {
        PartOfSpeech::Preposition(Word::new(base_form.to_string()))
    }

    fn preposition_base_form(&self) -> String {
        match self {
            PartOfSpeech::Preposition(word) => word.preposition_base_form(),
            _ => "".to_string(),
        }
    }

    fn case(&self) -> PrepositionCase {
        match self {
            PartOfSpeech::Preposition(word) => word.case(),
            _ => PrepositionCase::Accusative,
        }
    }
}


pub trait Conjunction {
    fn new(base_form: &str, conjunction_type: ConjunctionType, category: ConjunctionCategory) -> Self;
    fn base_form(&self) -> String;
    fn conjunction_type(&self) -> ConjunctionType;
    fn category(&self) -> ConjunctionCategory;
}

impl Conjunction for Word {
    fn new(base_form: &str, conjunction_type: ConjunctionType, category: ConjunctionCategory) -> Word {
        Word {
            base_form: base_form.to_string(),
        }
    }

    fn base_form(&self) -> String {
        self.to_string()
    }

    fn conjunction_type(&self) -> ConjunctionType {
        ConjunctionType::Coordinating
    }

    fn category(&self) -> ConjunctionCategory {
        ConjunctionCategory::Addition
    }
}

impl Conjunction for PartOfSpeech {
    fn new(base_form: &str, conjunction_type: ConjunctionType, category: ConjunctionCategory) -> PartOfSpeech {
        PartOfSpeech::Conjunction(Word::new(base_form.to_string()))
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Conjunction(word) => word.base_form(),
            _ => "".to_string(),
        }
    }

    fn conjunction_type(&self) -> ConjunctionType {
        match self {
            PartOfSpeech::Conjunction(word) => word.conjunction_type(),
            _ => ConjunctionType::Coordinating,
        }
    }

    fn category(&self) -> ConjunctionCategory {
        match self {
            PartOfSpeech::Conjunction(word) => word.category(),
            _ => ConjunctionCategory::Addition,
        }
    }
}


pub trait Interjection {
    fn new(word: &str, sentiment: Sentiment) -> Self;
    fn word(&self) -> String;
    fn sentiment(&self) -> Sentiment;
}

impl Interjection for Word {
    fn new(word: &str, sentiment: Sentiment) -> Word {
        Word {
            base_form: word.to_string(),
        }
    }

    fn word(&self) -> String {
        self.to_string()
    }

    fn sentiment(&self) -> Sentiment {
        Sentiment::Neutral
    }
}

impl Interjection for PartOfSpeech {
    fn new(word: &str, sentiment: Sentiment) -> PartOfSpeech {
        PartOfSpeech::Interjection(Word::new(word.to_string()))
    }

    fn word(&self) -> String {
        match self {
            PartOfSpeech::Interjection(word) => word.word(),
            _ => "".to_string(),
        }
    }

    fn sentiment(&self) -> Sentiment {
        match self {
            PartOfSpeech::Interjection(word) => word.sentiment(),
            _ => Sentiment::Neutral,
        }
    }
}


pub trait Article {
    fn new(form: &str, gender: Gender, number: Number) -> Self;
    fn form(&self) -> String;
    fn article_gender(&self) -> Gender;
    fn article_number(&self) -> Number;
}

impl Article for Word {
    fn new(form: &str, gender: Gender, number: Number) -> Word {
        Word {
            base_form: form.to_string(),
        }
    }

    fn form(&self) -> String {
        self.to_string()
    }

    fn article_gender(&self) -> Gender {
        Gender::Masculine
    }

    fn article_number(&self) -> Number {
        Number::Singular
    }
}

impl Article for PartOfSpeech {
    fn new(form: &str, gender: Gender, number: Number) -> PartOfSpeech {
        PartOfSpeech::Article(Word::new(form.to_string()))
    }

    fn form(&self) -> String {
        match self {
            PartOfSpeech::Article(word) => word.form(),
            _ => "".to_string(),
        }
    }

    fn article_gender(&self) -> Gender {
        match self {
            PartOfSpeech::Article(word) => word.article_gender(),
            _ => Gender::Neutral
        }
    }

    fn article_number(&self) -> Number {
        match self {
            PartOfSpeech::Article(word) => word.article_number(),
            _ => Number::Singular
        }
    }
}