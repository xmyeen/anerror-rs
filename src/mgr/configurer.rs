use std::{collections::{HashMap}};
// use config::ConfigError;
use serde::{Serialize,Deserialize};

pub type LangT = String;
pub type I18nCodeT = String;
pub type ParamNameT = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefinitionConfigurer {
    pub id: u32,
    pub code: String,
    pub i18n: HashMap<I18nCodeT, HashMap<LangT, (String, String)>>,
    pub param: Option<HashMap<ParamNameT, HashMap<LangT,String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorConfigurer {
    pub definitions: Vec<DefinitionConfigurer>,
}
