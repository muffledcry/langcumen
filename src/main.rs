#![allow(unused)]


mod word_tokens;
mod word_group_tokens;


use std::collections::VecDeque;
use std::fs;
use std::path::Path;

pub struct Processor {
    original_text: String,
    
}

pub struct Preprocessor {
    original_text: String, 
    all_tokens: VecDeque<String>,
    unique_words: VecDeque<String>, 
    sentence_tokens: VecDeque<String>,
}

impl Preprocessor {
    pub fn new(file_path: &Path) -> Preprocessor {
        let original_text = fs::read_to_string(file_path).expect("Unable to read file");
        let all_tokens = Preprocessor::get_all_tokens(&original_text);
        let unique_words = Preprocessor::get_unique_words(&all_tokens);
        let sentence_tokens = Preprocessor::get_sentence_tokens(&original_text);

        Preprocessor {
            original_text,
            all_tokens,
            unique_words,
            sentence_tokens,
        }
    }
    
    fn get_all_tokens(text: &str) -> VecDeque<String> {
        let mut tokens = VecDeque::new();

        let punctuation = vec![
            ",", ".", "!", "?", ";", ":", "'", "\"", "(", ")", "@", "#", "$", "%", "_", "~", "[", "]",
        ];
        let words: Vec<&str> = text.split(|c: char| c.is_whitespace()).collect();

        for word in words {
            if word.is_empty() {
                continue;
            }

            let first_char = word.chars().next().unwrap();
            if punctuation.contains(&first_char.to_string().as_str()) {
                tokens.push_back(first_char.to_string());
                if word.len() > 1 {
                    tokens.push_back(word[1..].to_string());
                }
            } else if punctuation.contains(&word[word.len() - 1..].to_string().as_str()) {
                tokens.push_back(word[..word.len() - 1].to_string());
                tokens.push_back(word[word.len() - 1..].to_string());
            } else {
                tokens.push_back(word.to_string());
            }
        }
        tokens
    }

    fn get_unique_words(tokens: &VecDeque<String>) -> VecDeque<String> {
        let mut unique_tokens: VecDeque<String> = VecDeque::new();

        for token in tokens {
            let stripped_token = token.trim_matches(|c: char| !c.is_alphabetic());
            if !stripped_token.is_empty() && !unique_tokens.contains(&stripped_token.to_lowercase()) {
                unique_tokens.push_back(stripped_token.to_lowercase());
            }
        }
        unique_tokens
    }

    fn get_sentence_tokens(text: &str) -> VecDeque<String> {
        let mut sentences = VecDeque::new();

        let punctuation = vec![".", "!", "?", ";"];
        let words: Vec<&str> = text.split_whitespace().collect();

        let mut sentence = String::new();
        for word in words {
            sentence.push_str(word);
            sentence.push(' ');

            let last_char = word.chars().last().unwrap();
            if punctuation.contains(&last_char.to_string().as_str()) {
                sentences.push_back(sentence.trim().to_string());
                sentence = String::new();
            }
        }
        if !sentence.is_empty() {
            sentences.push_back(sentence.trim().to_string());
        }
        sentences
    }
}


fn main() {
    let path = Path::new("romeo_and_juliet.txt");
    let preprocessor = Preprocessor::new(path);
}
