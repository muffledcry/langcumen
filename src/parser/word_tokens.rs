use super::word_enums::{Gender, Number, Modifies, Degree, Position, PrepositionCase};

pub struct Word;

pub struct Noun {
    singular: String,
    plural: String,
}

impl Noun {
    fn new(singular: &str) -> Noun {
        Noun {
            singular: singular.to_string(),
            plural: format!("{}s", singular),
        }
    }

    fn pluralize(&self) -> String {
        self.plural.clone()
    }

    fn singularize(&self) -> String {
        self.singular.clone()
    }
}

pub struct Pronoun {
    subject_form: String,
    object_form: String,
    possessive_form: String,
    reflexive_form: String,
    gender: Gender,
    number: Number,
}

pub struct Verb {
    base_form: String,
    past_tense: String,
    past_participle: String,
    present_tense: String,
    present_participle: String,
    third_person_singular: String,
    present_tense_singular: String,
    present_tense_plural: String,
    infinitive: String,
}

impl Verb {
    fn new(base_form: &str) -> Verb {
        Verb {
            base_form: base_form.to_string(),
            past_tense: format!("{}ed", base_form),
            past_participle: format!("{}ed", base_form),
            present_tense: format!("{}s", base_form),
            present_participle: format!("{}ing", base_form),
            third_person_singular: format!("{}s", base_form),
            present_tense_singular: format!("{}s", base_form),
            present_tense_plural: format!("{}", base_form),
            infinitive: format!("to {}", base_form),
        }
    }

    fn conjugate(&self, tense: &str) -> String {
        match tense {
            "past" => self.past_tense.clone(),
            "past_participle" => self.past_participle.clone(),
            "present" => self.present_tense.clone(),
            "present_participle" => self.present_participle.clone(),
            "third_person_singular" => self.third_person_singular.clone(),
            "present_tense_singular" => self.present_tense_singular.clone(),
            "present_tense_plural" => self.present_tense_plural.clone(),
            "infinitive" => self.infinitive.clone(),
            _ => self.base_form.clone(),
        }
    }
}

pub struct Adjective {
    base_form: String,
    gender: Gender,
    number: Number,
    degree: Degree,
    position: Position,
}

impl Adjective {
    fn new(
        base_form: &str,
        gender: Gender,
        number: Number,
        degree: Degree,
        position: Position,
    ) -> Adjective {
        Adjective {
            base_form: base_form.to_string(),
            gender,
            number,
            degree,
            position,
        }
    }

    fn adjust_degree(&self, degree: Degree) -> Adjective {
        Adjective {
            base_form: self.base_form.clone(),
            number: self.number,
            gender: self.gender,
            degree,
            position: self.position,
        }
    }

    fn get_gender(&self) -> Gender {
        self.gender
    }

    fn get_number(&self) -> Number {
        self.number
    }

    fn get_degree(&self) -> Degree {
        self.degree
    }

    fn get_position(&self) -> Position {
        self.position
    }
}

pub struct Adverb {
    base_form: String,
    modifies: Modifies,
    position: Position,
}

impl Adverb {
    fn new(base_form: &str, modifies: Modifies, position: Position) -> Adverb {
        Adverb {
            base_form: base_form.to_string(),
            modifies,
            position,
        }
    }

    fn get_modification(&self) -> Modifies {
        self.modifies
    }

    fn get_position(&self) -> Position {
        self.position
    }
}
pub struct Preposition {
    base_form: String,
    case: PrepositionCase,
}
pub struct Conjunction;
pub struct Interjection;
pub struct Article;
