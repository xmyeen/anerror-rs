/*use std::{convert::{From, Into}};*/
// use config::ConfigError;
// use serde::Deserialize;
use std::collections::{HashSet};

const MSG_SEP_DEF:&'static str = ". ";
const PARAM_SEP_DEF:&'static str = "";
const SEGMENT_SEP_DEF:&'static str = " | ";

pub type Result<T> = std::result::Result<T, AnError>; 

#[derive(Clone, Debug)]
struct AnErrorText {
    pub msgs: Vec<String>,
    pub prompts: Vec<String>,
    pub param_msgs: Vec<String>,
    pub param_prompts: Vec<String>,
}

impl AnErrorText {
    fn new(msg:&str, prompt:&str) -> Self {
        Self {
            msgs: vec![msg.to_string()],
            prompts: vec![prompt.to_string()],
            param_msgs: Vec::new(),
            param_prompts: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnError {
    id: u32,
    code: String,
    name: String,
    location: (String, u32),
    now: AnErrorText,
    ago: Vec<AnErrorText>,
}

impl AnError {
    pub (crate) fn new(id: u32, code:&str, name:&str, location: (&str, u32), (msg, prompt):(&str, &str)) -> Self {
        Self {
            id: id,
            code: code.to_string(),
            name: name.to_string(),
            location: (location.0.to_string(), location.1),
            now: AnErrorText::new(msg, prompt),
            ago: Vec::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn prompt(&self) -> String {
        // let mut prompts = self.now.prompt.split(MSG_SEP_DEF).collect::<Vec<&str>>();
        // prompts.extend(self.ago.iter().map(|et| et.prompt.split(MSG_SEP_DEF).collect::<Vec<&str>>()).flatten());
        // prompts = prompts.iter().map(|s| s.to_string()).unique();
        let prompt = self.ago.iter().fold(
            self.now.prompts.iter().map(|m| m.as_str()).collect::<HashSet<&str>>(), 
            |s, et| s.union(&et.prompts.iter().map(|m| m.as_str()).collect()).copied().collect()
        ).into_iter().collect::<Vec<&str>>().join(MSG_SEP_DEF);
        // self.ago.iter().for_each(|et| prompt.extend(et.prompts));

        // let mut param_prompts = vec![self.now.param_prompt.as_str()];
        // param_prompts.extend(self.ago.iter().map(|et| et.param_prompt.as_str()));
        // param_prompts = param_prompts.into_iter().filter(|p| !p.is_empty()).collect();
        let param_prompt = self.ago.iter().fold(
            self.now.param_prompts.iter().map(|m| m.as_str()).collect::<HashSet<&str>>(), 
            |s, et| s.union(&et.param_prompts.iter().map(|m| m.as_str()).collect()).copied().collect()
        ).into_iter().collect::<Vec<&str>>().join(PARAM_SEP_DEF);

        if !param_prompt.is_empty() {
            format!("(0x{:x}){}{sep}{}{sep}{}", self.id, self.name, prompt, param_prompt, sep = SEGMENT_SEP_DEF)
        } else {
            format!("(0x{:x}){}{sep}{}", self.id, self.name, prompt, sep = SEGMENT_SEP_DEF)
        }
    }

    pub fn msg(&self) -> String {
        //去重复
        let msg = self.ago.iter().fold(
            self.now.msgs.iter().map(|m| m.as_str()).collect::<HashSet<&str>>(), 
            |s, et| s.union(&et.msgs.iter().map(|m| m.as_str()).collect()).copied().collect()
        ).into_iter().collect::<Vec<&str>>().join(MSG_SEP_DEF);

        // let mut param_msgs = vec![self.now.param_msg.as_str()];
        // param_msgs.extend(self.ago.iter().map(|et| et.param_msg.as_str()));
        // param_msgs = param_msgs.into_iter().filter(|m| !m.is_empty()).collect();
        let param_msg = self.ago.iter().fold(
            self.now.param_msgs.iter().map(|m| m.as_str()).collect::<HashSet<&str>>(), 
            |s, et| s.union(&et.param_msgs.iter().map(|m| m.as_str()).collect()).copied().collect()
        ).into_iter().collect::<Vec<&str>>().join(PARAM_SEP_DEF);

        if !param_msg.is_empty() {
            format!("(0x{:x}){}{sep}{}{sep}{}", self.id, self.name, msg, param_msg, sep = SEGMENT_SEP_DEF)
        } else {
            format!("(0x{:x}){}{sep}{}", self.id, self.name, msg, sep = SEGMENT_SEP_DEF)
        }
    }

    pub fn file_name(&self) -> &str {
        self.location.0.as_ref()
    }

    pub fn file_number(&self) -> u32 {
        self.location.1
    }

    pub fn put_msg_to<T: std::fmt::Display>(mut self, msg: T) -> Self {
        // let mut datas = vec![self.now.msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.msg = datas.join(MSG_SEP_DEF);
        self.now.msgs.push(msg.to_string());
        self
    }

    pub fn put_msg_mut<T: std::fmt::Display>(&mut self, msg: T) -> &mut Self {
        // let mut datas = vec![self.now.msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.msg = datas.join(MSG_SEP_DEF);
        self.now.msgs.push(msg.to_string());
        self
    }

    pub fn put_msgs_to<T: std::fmt::Display, I: Iterator<Item = T>>(mut self, iter: I) -> Self {
        // let mut datas = vec![self.now.msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.msg = datas.join(MSG_SEP_DEF);
        self.now.msgs.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_msgs_mut<T: std::fmt::Display, I: Iterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        // let mut datas = vec![self.now.msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.msg = datas.join(MSG_SEP_DEF);
        self.now.msgs.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_param_msg_to<T: std::fmt::Display>(mut self, msg: T) -> Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.param_msgs.push(msg.to_string());
        self
    }

    pub fn put_param_msg_mut<T: std::fmt::Display>(&mut self, msg: T) -> &mut Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.param_msgs.push(msg.to_string());
        self
    }

    pub fn put_param_msgs_to<T: std::fmt::Display, I: Iterator<Item = T>>(mut self, iter: I) -> Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.param_msgs.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_param_msgs_mut<T: std::fmt::Display, I: Iterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.param_msgs.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_prompt_to<T: std::fmt::Display>(mut self, prompt: T) -> Self {
        // let mut datas = vec![self.now.prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.prompt = datas.join(MSG_SEP_DEF);
        self.now.prompts.push(prompt.to_string());
        self
    }

    pub fn put_prompt_mut<T: std::fmt::Display>(&mut self, prompt: T) -> &mut Self {
        // let mut datas = vec![self.now.prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.prompt = datas.join(MSG_SEP_DEF);
        self.now.prompts.push(prompt.to_string());
        self
    }

    pub fn put_prompts_to<T: std::fmt::Display, I: Iterator<Item = T>>(mut self, iter: I) -> Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.prompts.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_prompts_mut<T: std::fmt::Display, I: Iterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        // let mut datas = vec![self.now.param_msg];
        // datas.extend(msgs.iter().map(|m| m.to_string()).filter(|m| !m.is_empty()));
        // self.now.param_msg = datas.join("");
        self.now.prompts.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_param_prompt_to<T: std::fmt::Display>(mut self, prompt: T) -> Self {
        // let mut datas = vec![self.now.param_prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.param_prompt = datas.join("");
        self.now.param_prompts.push(prompt.to_string());
        self
    }

    pub fn put_param_prompt_mut<T: std::fmt::Display>(&mut self, prompt: T) -> &mut Self {
        // let mut datas = vec![self.now.param_prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.param_prompt = datas.join("");
        self.now.param_prompts.push(prompt.to_string());
        self
    }

    pub fn put_param_prompts_to<T: std::fmt::Display, I: Iterator<Item = T>>(mut self, iter: I) -> Self {
        // let mut datas = vec![self.now.param_prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.param_prompt = datas.join("");
        self.now.param_prompts.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn put_param_prompts_mut<T: std::fmt::Display, I: Iterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        // let mut datas = vec![self.now.param_prompt];
        // datas.extend(prompts.iter().map(|p| p.to_string()).filter(|p| !p.is_empty()));
        // self.now.param_prompt = datas.join("");
        self.now.param_prompts.extend(iter.into_iter().map(|i| i.to_string()));
        self
    }

    pub fn extend_to(mut self, an_error: Self) -> Self {
        self.ago.push(an_error.now);
        self
    }

    pub fn extend_mut(&mut self, an_error: Self) -> &mut Self {
        self.ago.push(an_error.now);
        self
    }
}

impl std::error::Error for AnError {
}

impl std::ops::Add for AnError {
    type Output = AnError;

    fn add(self, rhs: AnError) -> Self::Output {
        self.extend_to(rhs)
    }
}

impl std::fmt::Display for AnError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.now.param_msgs.is_empty() {
            write!(f, "[{}:{}]{sep}{}(0x{:x}){sep}{}{sep}{}", 
                self.location.0, self.location.1, 
                self.code, self.id, 
                self.now.msgs.join(MSG_SEP_DEF), 
                self.now.param_msgs.join(PARAM_SEP_DEF), 
                sep = SEGMENT_SEP_DEF
        )
        } else {
            write!(f, "[{}:{}]{sep}{}(0x{:x}){sep}{}", 
                self.location.0, self.location.1, 
                self.code, self.id, 
                self.now.msgs.join(MSG_SEP_DEF), 
                sep = SEGMENT_SEP_DEF
            )
        }
    }
}
