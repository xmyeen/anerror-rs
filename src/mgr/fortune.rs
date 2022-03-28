use std::{collections::{HashMap}};
use super::configurer::DefinitionConfigurer;
// use config::ConfigError;

#[derive(Debug)]
pub (crate) struct ErrorFortune {
    definition: HashMap<u32, DefinitionConfigurer>,
    lang: Option<String>,
}

impl ErrorFortune {
    pub fn new() -> Self {
        Self {
            definition: HashMap::new(),
            lang: None,
        }
    }

    pub fn get_definition(&self) -> &HashMap<u32, DefinitionConfigurer> {
        return &self.definition;
    }

    pub fn get_definition_mut(&mut self) -> &mut HashMap<u32, DefinitionConfigurer> {
        return &mut self.definition;
    }

    pub fn get_language<'a>(&'a self, default_language:&'a str) -> &'a str {
        return self.lang.as_ref().map(|s| s.as_str()).unwrap_or(default_language);
    }

    pub fn change_language(&mut self, language:&str) {
        self.lang = Some(language.to_string());
    }
}