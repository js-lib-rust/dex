use crate::error::{AppError, Result};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize, Debug)]
pub struct Definition {
    word: String,
    // key is a space separated string of all word's flexions, in both UTF-8 and ASCII formats
    key: String,
    pub meanings: Vec<Meaning>,
    expressions: Vec<Expression>,
}

impl Definition {
    pub fn builder() -> DefinitionBuilder {
        DefinitionBuilder::new()
    }
}
pub struct DefinitionBuilder {
    word: Option<String>,
    keys: HashSet<String>,
    meanings: Vec<Meaning>,
    expressions: Vec<Expression>,
}

impl DefinitionBuilder {
    fn new() -> Self {
        DefinitionBuilder {
            word: None,
            keys: HashSet::new(),
            meanings: Vec::new(),
            expressions: Vec::new(),
        }
    }

    pub fn word(mut self, word: &str) -> Self {
        self.word = Some(word.to_string());
        self
    }

    pub fn key(mut self, key: &str) -> Self {
        self.keys.insert(key.to_string());
        self
    }

    pub fn meaning(mut self, meaning: Meaning) -> Self {
        self.meanings.push(meaning);
        self
    }

    pub fn expression(mut self, expression: Expression) -> Self {
        self.expressions.push(expression);
        self
    }

    pub fn build(mut self) -> Result<Definition> {
        let word = self.word.ok_or(AppError::Fatal("definition word"))?;
        self.keys.insert(word.clone());

        Ok(Definition {
            word,
            key: self.keys.into_iter().collect::<Vec<String>>().join(" "),
            meanings: self.meanings,
            expressions: self.expressions,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct Expression {
    phrase: String,
    definition: String,
    examples: Vec<Example>,
}

impl Expression {
    pub fn new(phrase: &str, definition: &str) -> Self {
        Self {
            phrase: phrase.to_string(),
            definition: definition.to_string(),
            examples: Vec::new(),
        }
    }

    pub fn add_example(&mut self, example: Example) {
        self.examples.push(example);
    }
}

#[derive(Serialize, Debug)]
pub struct Example {
    text: String,
    source: Option<String>,
}

impl Example {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            source: None,
        }
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = Some(source.to_string());
    }
}

#[derive(Serialize, Debug)]
pub struct Meaning {
    definition: String,
    examples: Vec<Example>,
}

impl Meaning {
    pub fn new(definition: &str) -> Self {
        Self {
            definition: definition.to_string(),
            examples: Vec::new(),
        }
    }

    pub fn add_example(&mut self, example: Example) {
        self.examples.push(example);
    }
}
