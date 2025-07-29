use async_trait::async_trait;
use color_eyre::eyre::OptionExt;
use color_eyre::{Result, eyre::eyre};
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Deserialize, Clone)]
pub struct WordMetadata {
    #[serde(rename = "MEANINGS", deserialize_with = "deserialize_meanings")]
    pub meanings: Vec<String>,
    #[serde(rename = "ANTONYMS")]
    pub antonyms: Vec<String>,
    #[serde(rename = "SYNONYMS")]
    pub synonyms: Vec<String>,
}

// Custom deserializer for meanings
/*
{
    "WORD": {
        "MEANINGS": [
            [
                "Type",
                "Definition",
            ],
            ...
        ],
        "ANTONYMS": [],
        "SYNONYMS": []
    },
  ...
}
*/
fn deserialize_meanings<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let arrays: Vec<Vec<serde_json::Value>> = Deserialize::deserialize(deserializer)?;
    Ok(arrays
        .iter()
        .filter_map(|arr| arr.get(1)?.as_str())
        .map(String::from)
        .collect())
}

#[derive(Clone)]
pub struct Word {
    pub word: String,
    pub metadata: WordMetadata,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

#[async_trait]
pub trait WordService: Send + Sync {
    async fn get_random_word(&self, length: usize) -> Result<Word>;
    async fn validate_word(&self, word: &str) -> Result<bool>;
    async fn get_word_metadata(&self, word: &str) -> Result<WordMetadata>;
}

pub struct DictionaryService {
    words_by_length: HashMap<usize, Vec<Word>>,
    dictionary: HashMap<String, WordMetadata>,
}

impl DictionaryService {
    pub async fn new() -> Result<Self> {
        // Load and parse JSON file (try from URL, otherwise read from local file)
        let json_str: std::result::Result< HashMap<String, WordMetadata>, reqwest::Error> = reqwest::get("https://raw.githubusercontent.com/nightblade9/simple-english-dictionary/refs/heads/main/processed/filtered.json").await?.json().await;
        let dictionary: HashMap<String, WordMetadata> = json_str.unwrap_or(serde_json::from_str(
            include_str!("../assets/english_wordlist.json"),
        )?);

        // Precompute words by length
        let mut words_by_length = HashMap::new();
        for (word, metadata) in dictionary.iter() {
            if !word.chars().all(|c| c.is_ascii_alphabetic()) {
                continue;
            }

            let len = word.len();
            words_by_length
                .entry(len)
                .or_insert_with(Vec::new)
                .push(Word {
                    word: word.to_string(),
                    metadata: metadata.clone(),
                });
        }

        Ok(Self {
            words_by_length,
            dictionary,
        })
    }
}

#[async_trait]
impl WordService for DictionaryService {
    async fn get_random_word(&self, length: usize) -> Result<Word> {
        let words = self
            .words_by_length
            .get(&length)
            .ok_or_eyre(format!("No words of length {length}"))?;

        let word = words
            .choose(&mut rand::rng())
            .ok_or_eyre(format!("No word available for length {length}"))?;

        Ok(word.clone())
    }

    async fn validate_word(&self, word: &str) -> Result<bool> {
        Ok(self.dictionary.contains_key(&word.to_ascii_uppercase()))
    }

    async fn get_word_metadata(&self, word: &str) -> Result<WordMetadata> {
        Ok(self
            .dictionary
            .get(&word.to_ascii_uppercase())
            .cloned()
            .ok_or_else(|| eyre!("No metadata available for word {}", word))?)
    }
}
