use async_trait::async_trait;
use color_eyre::Result;
use color_eyre::eyre::OptionExt;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Deserialize, Clone)]
pub struct WordData {
    #[serde(deserialize_with = "deserialize_words")]
    pub words: Vec<Word>,
}

// Custom deserializer for json words
/*
{
    "word": "definition",
    ...
}
*/
fn deserialize_words<'de, D>(deserializer: D) -> Result<Vec<Word>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let arrays: HashMap<String, String> = Deserialize::deserialize(deserializer)?;

    Ok(arrays
        .iter()
        .map(|(word, meanings)| {
            let meanings: Vec<String> = meanings.split("--").map(str::to_string).collect();
            Word {
                word: word.to_string().to_ascii_uppercase(),
                meanings,
            }
        })
        .collect::<Vec<Word>>())
}

#[derive(Clone, Deserialize, Debug)]
pub struct Word {
    pub word: String,
    pub meanings: Vec<String>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

#[async_trait]
pub trait WordService: Send + Sync {
    async fn get_random_word(&self, length: usize) -> Result<Word>;
    async fn get_words_by_length(&self, length: usize) -> Result<Vec<Word>>;
    async fn validate_word(&self, word: &str) -> Result<bool>;
}

pub struct DictionaryService {
    dictionary: Vec<Word>,
}

impl DictionaryService {
    pub async fn new() -> Result<Self> {
        // Load and parse JSON file
        let dictionary: WordData = serde_json::from_str(include_str!("../assets/dictionary.json"))?;

        Ok(Self {
            dictionary: dictionary.words,
        })
    }
}

#[async_trait]
impl WordService for DictionaryService {
    async fn get_random_word(&self, length: usize) -> Result<Word> {
        let words = self.get_words_by_length(length).await?;

        let word = words
            .choose(&mut rand::rng())
            .ok_or_eyre(format!("No word available for length {length}"))?;

        Ok(word.clone())
    }

    async fn validate_word(&self, word: &str) -> Result<bool> {
        Ok(self
            .dictionary
            .iter()
            .any(|w| w.word == word.to_ascii_uppercase()))
    }

    async fn get_words_by_length(&self, length: usize) -> Result<Vec<Word>> {
        Ok(self
            .dictionary
            .iter()
            .filter(|x: &&Word| x.word.len() == length)
            .cloned()
            .collect())
    }
}
