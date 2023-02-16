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

pub struct Word;


pub trait Noun {
    fn new(singular: &str) -> Self;
    fn pluralize(&self) -> String;
    fn singularize(&self) -> String;
}

impl Noun for Word {
    fn new(singular: &str) -> Word {
        Word
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
        PartOfSpeech::Noun(Word::new(singular))
    }

    fn pluralize(&self) -> String {
        match self {
            PartOfSpeech::Noun(word) => word.pluralize(),
            // other cases
        }
    }

    fn singularize(&self) -> String {
        match self {
            PartOfSpeech::Noun(word) => word.singularize(),
            // other cases
        }
    }
}


pub trait Pronoun {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, gender: Gender, number: Number) -> Self;
    fn subject_form(&self) -> String;
    fn object_form(&self) -> String;
    fn possessive_form(&self) -> String;
    fn reflexive_form(&self) -> String;
    fn gender(&self) -> Gender;
    fn number(&self) -> Number;
}

impl Pronoun for Word {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, gender: Gender, number: Number) -> Word {
        Word
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

    fn gender(&self) -> Gender {
        Gender::Neuter
    }

    fn number(&self) -> Number {
        Number::Singular
    }
}

impl Pronoun for PartOfSpeech {
    fn new(subject_form: &str, object_form: &str, possessive_form: &str, reflexive_form: &str, gender: Gender, number: Number) -> PartOfSpeech {
        PartOfSpeech::Pronoun(Word::new(subject_form, object_form, possessive_form, reflexive_form, gender, number))
    }

    fn subject_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.subject_form(),
            // other cases
        }
    }

    fn object_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.object_form(),
            // other cases
        }
    }

    fn possessive_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.possessive_form(),
            // other cases
        }
    }

    fn reflexive_form(&self) -> String {
        match self {
            PartOfSpeech::Pronoun(word) => word.reflexive_form(),
            // other cases
        }
    }

    fn gender(&self) -> Gender {
        match self {
            PartOfSpeech::Pronoun(word) => word.gender(),
            // other cases
        }
    }

    fn number(&self) -> Number {
        match self {
            PartOfSpeech::Pronoun(word) => word.number(),
            // other cases
        }
    }
}


pub trait Verb {
    fn new(base_form: &str) -> Self;
    fn conjugate(&self, tense: &str) -> String;
    fn base_form(&self) -> String;
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
        Word
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
            _ => self.base_form(),
        }
    }

    fn base_form(&self) -> String {
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
        PartOfSpeech::Verb(Word::new(base_form))
    }

    fn conjugate(&self, tense: &str) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.conjugate(tense),
            // other cases
        }
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.base_form(),
            // other cases
        }
    }

    fn past_tense(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.past_tense(),
            // other cases
        }
    }

    fn past_participle(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.past_participle(),
            // other cases
        }
    }

    fn present_tense(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense(),
            // other cases
        }
    }

    fn present_participle(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_participle(),
            // other cases
        }
    }

    fn third_person_singular(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.third_person_singular(),
            // other cases
        }
    }

    fn present_tense_singular(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense_singular(),
            // other cases
        }
    }

    fn present_tense_plural(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.present_tense_plural(),
            // other cases
        }
    }

    fn infinitive(&self) -> String {
        match self {
            PartOfSpeech::Verb(word) => word.infinitive(),
            // other cases
        }
    }
}


pub trait Adjective {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> Self;
    fn adjust_degree(&self, degree: Degree) -> Self;
    fn base_form(&self) -> String;
    fn gender(&self) -> Gender;
    fn number(&self) -> Number;
    fn degree(&self) -> Degree;
    fn position(&self) -> Position;
}

impl Adjective for Word {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> Word {
        Word
    }

    fn adjust_degree(&self, degree: Degree) -> Word {
        Word
    }

    fn base_form(&self) -> String {
        self.to_string()
    }

    fn gender(&self) -> Gender {
        Gender::Neuter
    }

    fn number(&self) -> Number {
        Number::Singular
    }

    fn degree(&self) -> Degree {
        Degree::Positive
    }

    fn position(&self) -> Position {
        Position::Attributive
    }
}

impl Adjective for PartOfSpeech {
    fn new(base_form: &str, gender: Gender, number: Number, degree: Degree, position: Position) -> PartOfSpeech {
        PartOfSpeech::Adjective(Word::new(base_form, gender, number, degree, position))
    }

    fn adjust_degree(&self, degree: Degree) -> PartOfSpeech {
        match self {
            PartOfSpeech::Adjective(word) => PartOfSpeech::Adjective(word.adjust_degree(degree)),
            // other cases
        }
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Adjective(word) => word.base_form(),
            // other cases
        }
    }

    fn gender(&self) -> Gender {
        match self {
            PartOfSpeech::Adjective(word) => word.gender(),
            // other cases
        }
    }

    fn number(&self) -> Number {
        match self {
            PartOfSpeech::Adjective(word) => word.number(),
            // other cases
        }
    }

    fn degree(&self) -> Degree {
        match self {
            PartOfSpeech::Adjective(word) => word.degree(),
            // other cases
        }
    }

    fn position(&self) -> Position {
        match self {
            PartOfSpeech::Adjective(word) => word.position(),
            // other cases
        }
    }
}


pub trait Adverb {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> Self;
    fn base_form(&self) -> String;
    fn modifies(&self) -> Modifies;
    fn position(&self) -> Position;
}

impl Adverb for Word {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> Word {
        Word
    }

    fn base_form(&self) -> String {
        self.to_string()
    }

    fn modifies(&self) -> Modifies {
        Modifies::Verb
    }

    fn position(&self) -> Position {
        Position::After
    }
}

impl Adverb for PartOfSpeech {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> PartOfSpeech {
        PartOfSpeech::Adverb(Word::new(base_form, modifies, position))
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Adverb(word) => word.base_form(),
            // other cases
        }
    }

    fn modifies(&self) -> Modifies {
        match self {
            PartOfSpeech::Adverb(word) => word.modifies(),
            // other cases
        }
    }

    fn position(&self) -> Position {
        match self {
            PartOfSpeech::Adverb(word) => word.position(),
            // other cases
        }
    }
}


pub trait Preposition {
    fn new(base_form: &str, case: PrepositionCase) -> Self;
    fn base_form(&self) -> String;
    fn case(&self) -> PrepositionCase;
}

impl Preposition for Word {
    fn new(base_form: &str, case: PrepositionCase) -> Word {
        Word
    }

    fn base_form(&self) -> String {
        self.to_string()
    }

    fn case(&self) -> PrepositionCase {
        PrepositionCase::Accusative
    }
}

impl Preposition for PartOfSpeech {
    fn new(base_form: &str, case: PrepositionCase) -> PartOfSpeech {
        PartOfSpeech::Preposition(Word::new(base_form, case))
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Preposition(word) => word.base_form(),
            // other cases
        }
    }

    fn case(&self) -> PrepositionCase {
        match self {
            PartOfSpeech::Preposition(word) => word.case(),
            // other cases
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
        Word
    }

    fn base_form(&self) -> String {
        self.to_string()
    }

    fn conjunction_type(&self) -> ConjunctionType {
        ConjunctionType::Coordinating
    }

    fn category(&self) -> ConjunctionCategory {
        ConjunctionCategory::Additive
    }
}

impl Conjunction for PartOfSpeech {
    fn new(base_form: &str, conjunction_type: ConjunctionType, category: ConjunctionCategory) -> PartOfSpeech {
        PartOfSpeech::Conjunction(Word::new(base_form, conjunction_type, category))
    }

    fn base_form(&self) -> String {
        match self {
            PartOfSpeech::Conjunction(word) => word.base_form(),
            // other cases
        }
    }

    fn conjunction_type(&self) -> ConjunctionType {
        match self {
            PartOfSpeech::Conjunction(word) => word.conjunction_type(),
            // other cases
        }
    }

    fn category(&self) -> ConjunctionCategory {
        match self {
            PartOfSpeech::Conjunction(word) => word.category(),
            // other cases
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
        Word
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
        PartOfSpeech::Interjection(Word::new(word, sentiment))
    }

    fn word(&self) -> String {
        match self {
            PartOfSpeech::Interjection(word) => word.word(),
            // other cases
        }
    }

    fn sentiment(&self) -> Sentiment {
        match self {
            PartOfSpeech::Interjection(word) => word.sentiment(),
            // other cases
        }
    }
}


pub trait Article {
    fn new(form: &str, gender: Gender, number: Number) -> Self;
    fn form(&self) -> String;
    fn gender(&self) -> Gender;
    fn number(&self) -> Number;
}

impl Article for Word {
    fn new(form: &str, gender: Gender, number: Number) -> Word {
        Word
    }

    fn form(&self) -> String {
        self.to_string()
    }

    fn gender(&self) -> Gender {
        Gender::Masculine
    }

    fn number(&self) -> Number {
        Number::Singular
    }
}

impl Article for PartOfSpeech {
    fn new(form: &str, gender: Gender, number: Number) -> PartOfSpeech {
        PartOfSpeech::Article(Word::new(form, gender, number))
    }

    fn form(&self) -> String {
        match self {
            PartOfSpeech::Article(word) => word.form(),
            // other cases
        }
    }

    fn gender(&self) -> Gender {
        match self {
            PartOfSpeech::Article(word) => word.gender(),
            // other cases
        }
    }

    fn number(&self) -> Number {
        match self {
            PartOfSpeech::Article(word) => word.number(),
            // other cases
        }
    }
}