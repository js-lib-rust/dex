use deunicode::deunicode;
use std::collections::HashSet;

use crate::error::Result;
use crate::model::{Definition, Example, Expression, Meaning};
use mysql::prelude::*;
use mysql::*;
use regex::Regex;

#[derive(Debug)]
struct Record {
    id: u32,
    parent_id: u32,
    text: String,
    kind: u8,
}

pub struct Database {
    connection: PooledConn,
}

impl Database {
    pub fn try_new(url: &str) -> Result<Self> {
        let pool = Pool::new(url)?;
        let connection = pool.get_conn()?;

        Ok(Self { connection })
    }

    pub fn next_word(&mut self, base_id: u32) -> Option<(u32, String)> {
        let query = format!(
            "SELECT e.id,e.description AS word FROM entry e \
        JOIN treeentry te ON e.id=te.entryId \
        JOIN tree t ON te.treeId=t.id \
        JOIN meaning m ON t.id=m.treeId \
        WHERE e.id>{base_id} AND e.structuristId<>0 \
        ORDER BY e.id \
        LIMIT 1"
        );

        let row: Row = self.connection.query_first(&query).ok()??;
        let id: u32 = row.get("id")?;
        let word: String = row.get("word")?;
        let word = word.split_whitespace().next()?.to_string();
        Some((id, word))
    }

    pub fn query(&mut self, definition_id: u32, word: String) -> Result<Definition> {
        println!("definition_id: {definition_id}, word: {word}");

        let definition_query = format!(
            "SELECT m.id AS id,m.parentId AS parent_id,m.internalRep AS text,m.type AS kind FROM entry e \
        JOIN treeentry te ON e.id=te.entryId \
        JOIN tree t ON te.treeId=t.id \
        JOIN meaning m ON t.id=m.treeId \
        WHERE e.id={definition_id} AND e.structuristId<>0 \
        ORDER BY m.displayOrder"
        );

        let inflections_query = format!(
            "SELECT DISTINCT f.formUtf8General FROM entry e 
        JOIN entrylexeme el ON e.id=el.entryId \
        JOIN inflectedform f ON el.lexemeId=f.lexemeId \
        WHERE e.id={definition_id} and e.structuristId<>0"
        );

        let records: Vec<Record> =
            self.connection
                .query_map(definition_query, |(id, parent_id, text, kind)| Record {
                    id,
                    parent_id,
                    text,
                    kind,
                })?;

        let inflections: Vec<String> = self
            .connection
            .query_map(inflections_query, |inflection| inflection)?;

        self.records_to_definition(word, inflections, records)
    }

    fn records_to_definition(
        &mut self,
        word: String,
        inflections: Vec<String>,
        records: Vec<Record>,
    ) -> Result<Definition> {
        let mut keys = HashSet::new();
        for inflection in inflections {
            let key = inflection.to_lowercase();
            keys.insert(deunicode(&key));
            keys.insert(key);
        }

        let mut definition_builder = Definition::builder().word(&word);
        for key in keys {
            definition_builder = definition_builder.key(&key);
        }

        let r_missing_definition = Regex::new(r"^(\(.+\)|.+:)$")?;
        let r_incomplete_meaning = Regex::new(r"^\$\((.+)\)\$\s*$")?;
        for item in DefIterator::new(records) {
            let mut definition_type = match item.definition.as_str() {
                "" => {
                    let Some(synonymous) = self.synonymous(item.id) else {
                        continue;
                    };
                    let definition = if let Some(first_char) = synonymous.chars().next() {
                        let rest = &synonymous[first_char.len_utf8()..];
                        format!("{}{}.", first_char.to_uppercase(), rest)
                    } else {
                        String::new()
                    };
                    DefType::Meaning(Meaning::new(&definition))
                }

                s if r_incomplete_meaning.is_match(s) => {
                    let Some(synonymous) = self.synonymous(item.id) else {
                        continue;
                    };
                    let definition = if let Some(first_char) = synonymous.chars().next() {
                        let rest = &synonymous[first_char.len_utf8()..];
                        format!("{} {}{}.", &self.str(s), first_char.to_uppercase(), rest)
                    } else {
                        String::new()
                    };
                    DefType::Meaning(Meaning::new(&definition))
                }

                s if r_missing_definition.is_match(s) => {
                    let Some(synonymous) = self.synonymous(item.id) else {
                        continue;
                    };
                    let definition = if let Some(first_char) = synonymous.chars().next() {
                        let rest = &synonymous[first_char.len_utf8()..];
                        if s.ends_with(":") {
                            format!("{} {}{}.", &self.str(s), first_char, rest)
                        } else {
                            format!("{} {}{}.", &self.str(s), first_char.to_uppercase(), rest)
                        }
                    } else {
                        String::new()
                    };
                    DefType::Meaning(Meaning::new(&definition))
                }

                s if s.starts_with("$") => {
                    let Some((phrase, definition)) = self.parse_expression(item.id, s) else {
                        continue;
                    };
                    DefType::Expression(Expression::new(&self.str(&phrase), &self.str(&definition)))
                }

                _ => DefType::Meaning(Meaning::new(&self.str(&item.definition))),
            };

            for example in item.examples {
                if let Some(example) = self.parse_example(&example) {
                    definition_type.add_example(example);
                }
            }

            match definition_type {
                DefType::Meaning(meaning) => {
                    definition_builder = definition_builder.meaning(meaning);
                }
                DefType::Expression(expression) => {
                    definition_builder = definition_builder.expression(expression);
                }
            }
        }

        definition_builder.build()
    }

    fn parse_expression(&mut self, record_id: u32, expression: &str) -> Option<(String, String)> {
        let r = Regex::new(r"^\$([^=]+) =\$?(?: (.+?)\.?)?$").ok()?;
        if let Some(captures) = r.captures(expression) {
            let phrase = captures.get(1).map(|m| m.as_str().to_string())?;
            let definition = match captures.get(2) {
                Some(definition) => definition.as_str().to_string(),
                None => format!("(sinonim) {}", self.synonymous(record_id)?),
            };
            return Some((phrase, definition));
        }

        let r = Regex::new(r"^\$(.+)\$ (se spune.+?)\.?$").ok()?;
        if let Some(captures) = r.captures(expression) {
            let phrase = captures.get(1).map(|m| m.as_str().to_string())?;
            let definition = captures.get(2).map(|m| m.as_str().to_string())?;
            return Some((phrase, definition));
        }

        let r = Regex::new(r"\[(\d+)\]").ok()?;
        if let Some(definition_id) = r
            .captures(expression)
            .and_then(|cap| cap.get(1))
            .and_then(|id| id.as_str().parse::<u32>().ok())
        {
            let definition_query =
                format!("SELECT internalRep FROM meaning WHERE id={definition_id}");
            let definition: Option<String> = self.connection.query_first(definition_query).ok()?;
            if let Some(mut definition) = definition {
                let r_definition = Regex::new(r"^\$[^=]+ =(?: (.+?)\.?)?$").ok()?;
                if let Some(captures) = r_definition.captures(&definition) {
                    definition = captures.get(1).map(|m| m.as_str())?.to_string();
                    return Some((expression.to_string(), definition));
                }
            };
        };

        println!("--- rejected expression: record_id: {record_id}, expression: {expression}");
        None
    }

    fn parse_example(&self, example: &str) -> Option<Example> {
        let r = Regex::new(r"^([$\[\(].+\$(?: \[.+\])?\.?)\s?(\(?[\w\-.,]{2,}\)?.*)?$").ok()?;
        if let Some(captures) = r.captures(example) {
            let text = captures.get(1).map(|m| m.as_str())?;
            let mut example = Example::new(&self.str(text));
            if let Some(source) = captures.get(2).map(|m| m.as_str()) {
                example.set_source(source);
            }
            return Some(example);
        }
        println!("--- rejected example: {example}");
        None
    }

    fn synonymous(&mut self, meaning_id: u32) -> Option<String> {
        let query = format!(
            "SELECT t.description FROM relation r JOIN tree t ON r.treeId=t.id WHERE r.meaningId={meaning_id}"
        );
        let synonymous: Vec<String> = self.connection.query(query).ok()?;

        let filtered: Vec<&str> = synonymous
            .iter()
            .map(|s| s.split(',').next().unwrap_or(s).trim())
            .collect();

        if filtered.is_empty() {
            None
        } else {
            Some(filtered.join(", "))
        }
    }

    fn str(&self, text: &str) -> String {
        let r = Regex::new(r"\[(\d+)\]").unwrap();
        let text = r.replace_all(text, "").to_string();

        let r = Regex::new(r"\u{0022}(.*?)\u{0022}").unwrap();
        let text = r.replace_all(&text, "\u{2018}$1\u{2019}");

        let r = Regex::new(r"\u{00AB}(.*?)\u{00BB}").unwrap();
        let text = r.replace_all(&text, "\u{2018}$1\u{2019}");

        return text.replace("$", "");
    }
}

enum DefType {
    Meaning(Meaning),
    Expression(Expression),
}

impl DefType {
    fn add_example(&mut self, example: Example) {
        match self {
            DefType::Meaning(meaning) => meaning.add_example(example),
            DefType::Expression(expression) => expression.add_example(example),
        }
    }
}

struct DefItem {
    id: u32,
    definition: String,
    examples: Vec<String>,
}

struct DefIterator {
    records: Vec<Record>,
    definition_id: usize,
}

impl DefIterator {
    fn new(records: Vec<Record>) -> Self {
        Self {
            records,
            definition_id: 0,
        }
    }
}

impl Iterator for DefIterator {
    type Item = DefItem;

    fn next(&mut self) -> Option<Self::Item> {
        let definition_record = loop {
            let record = self.records.get(self.definition_id)?;
            self.definition_id += 1;
            if matches!(record.kind, 0 | 5) {
                break record;
            }
        };

        let id = definition_record.id;
        let definition = definition_record.text.clone();

        // we do not have guarantees regarding records order so need to full scan
        let examples: Vec<String> = self
            .records
            .iter()
            .filter(|r| r.kind == 2 && r.parent_id == id)
            .map(|r| r.text.clone())
            .collect();

        Some(DefItem {
            id,
            definition,
            examples,
        })
    }
}
