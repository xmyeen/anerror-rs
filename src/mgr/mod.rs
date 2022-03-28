use std::{/*fmt::Arguments,*/fs::File, io::{Read}, /*convert::{From, Into},*/sync::{Arc, RwLock, Once}, collections::{HashMap, hash_map::Entry}};
// use config::ConfigError;
extern crate regex;

mod configurer;
use crate::svc::ErrDefs;

pub use self::configurer::{LangT, ParamNameT, I18nCodeT, ErrorConfigurer, DefinitionConfigurer};

mod error;
pub use error::{AnError, Result};

mod fortune;

const BASIC_ERROR_DEFAULT:&'static str = "
[[definitions]]
id = 0x0
code = \"SUCCESS\"
    [definitions.i18n.0]
    en_US = [\"Succeed\", \"Succeed\"]
    zh_CN = [\"成功\", \"成功\"]
    zh_TW = [\"成功\", \"成功\"]
    zh_HK = [\"成功\", \"成功\"]
    fr_FR = [\"Succès\", \"Succès\"]
    de_CH = [\"успех\", \"успех\"]
    ja_JP = [\"成功\", \"成功\"]
    es_LA = [\"éxito\", \"éxito\"]
[[definitions]]
id = 0x1
code = \"FAILED\"
    [definitions.i18n.0]
    en_US = [\"Faild\", \"Faild\"]
    zh_CN = [\"失败\", \"失败\"]
    zh_TW = [\"失敗\", \"失敗\"]
    zh_HK = [\"失敗\", \"失敗\"]
    fr_FR = [\"Échec\", \"Échec\"]
    de_CH = [\"неудача\", \"неудача\"]
    ja_JP = [\"失敗\", \"失敗\"]
    es_LA = [\"Fracaso\", \"Fracaso\"]
";

/// 错误码管理
#[derive(Clone)]
pub struct ErrorHolder {
    inner: Arc<RwLock<fortune::ErrorFortune>>,
}

/// 错误码管理的单例的方法
impl ErrorHolder {
    pub fn singleton() -> Self {
        static mut SINGLETON: *const ErrorHolder = 0 as *const ErrorHolder;
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let singleton = Self {
                    inner: Arc::new(RwLock::new(fortune::ErrorFortune::new())),
                };

                // Put it in the heap so it can outlive this call
                SINGLETON = std::mem::transmute(Box::new(singleton));
            });

            // Now we give out a copy of the data that is safe to use concurrently.
            (*SINGLETON).clone()
        }
    }

    // fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
    //     match self.inner.read() {
    //         Ok(fortune) => { 
    //             let toml_str = toml::to_string(&*configurer).unwrap();
    //             writer.write(toml_str.as_bytes())
    //         },
    //         Err(e) => { panic!("Can't serialize: {}", e) },
    //     }
    // }

    fn is_ok(id_n:u32) -> bool {
        ErrDefs::SUCCESS as u32 == id_n 
    }

    fn merge_i18n(&self, origin_i18n_map: &mut HashMap<I18nCodeT, HashMap<LangT, (String, String)>>, i18n_map: HashMap<I18nCodeT, HashMap<LangT, (String,String)>>) {
        for (i18n_code, i18n) in i18n_map.into_iter() {
            // let i18n = i18n_configurer_map.remove(i18n_code).unwrap();
            // 根据本地编码，进行合并覆盖。
            match origin_i18n_map.entry(i18n_code.clone()) {
                //合并翻译信息，从导入的资源覆盖内存的翻译。
                Entry::Occupied(mut origin_i18n) => {
                    for (k,v) in i18n.into_iter() {
                        origin_i18n.get_mut().insert(k, v);
                    }
                },

                //内存没有此翻译，则添加此翻译
                Entry::Vacant(entry) => {
                    entry.insert(i18n);
                },
            }
        }
    }

    fn merge_param_i18n(&self, origin_param:&mut HashMap<ParamNameT, HashMap<LangT,String>>, param: HashMap<ParamNameT, HashMap<LangT,String>>) {
        for (param_name, param_i18n) in param {
            match origin_param.entry(param_name) {
                //合并翻译信息，从导入的资源覆盖内存的翻译。
                Entry::Occupied(mut origin_param_i18n) => {
                    for (k,v) in param_i18n.into_iter() {
                        origin_param_i18n.get_mut().insert(k, v);
                    }
                },

                //内存没有此翻译，则添加此翻译
                Entry::Vacant(entry) => {
                    entry.insert(param_i18n);
                },
            }
        }
    }

    fn merge_defintion(&self, origin_definition: &mut DefinitionConfigurer, definition: DefinitionConfigurer) {
        //合并翻译
        self.merge_i18n(&mut origin_definition.i18n, definition.i18n);

        //合并词条
        if let Some(param) = definition.param {
            if let Some(origin_param) = origin_definition.param.as_mut() {
                //根据本地编码，进行合并覆盖。
                self.merge_param_i18n(origin_param, param);
            } else {
                origin_definition.param = Some(param)
            }
        }
    }

    fn merge_configurer(&mut self, origin_configurer: &mut ErrorConfigurer, configurer: ErrorConfigurer) {
        // let mut configurer = configurer;
        // let definitions = std::cell::Cell::new(configurer.definitions);
        // let definitions = std::boxed::Box::new(configurer.definitions);

        let mut definitions = std::boxed::Box::new(Vec::new());
        definitions.extend(configurer.definitions);

        for origin_definition in origin_configurer.definitions.iter_mut() {
            // let definitions = definitions.drain_filter(|definition| origin_definition.id == definition.id);
            // let (match_definitions, mismatch_defintions):(Vec<DefinitionConfigurer>, Vec<DefinitionConfigurer>) = definitions
            //     .into_iter()
            //     .partition(|d| d.id == origin_definition.id);
            // definitions.extend(mismatch_defintions);

            let remains = definitions.clone().into_iter()
                .map(|definition|{
                    if origin_definition.id == definition.id {
                        origin_definition.code = definition.code.clone();
                        self.merge_defintion(origin_definition, definition);
                        None
                    } else {
                        Some(definition)
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<DefinitionConfigurer>>();

            definitions.clear();
            definitions.extend(remains);
        }

        if !definitions.is_empty() {
            origin_configurer.definitions.extend(definitions.into_iter());
        }
    }

    // pub fn put(&mut self, errors: Vec<AnError>) {
    //     match self.inner.write() {
    //         Ok(mut m) => {
    //             for error in errors { 
    //                 m.insert(error.id, error); 
    //             }
    //         },
    //         Err(_) => {}
    //     };
    // }

    pub fn get<N: Into<u32>>(&self, id: N, i18n_code: Option<&str>, loc: (&str, u32), loc_msg: &str, loc_param: HashMap<&str, String>) -> AnError {
        let id_n = id.into();

        if Self::is_ok(id_n) {
            panic!("Create create error object that error id equal 0");
        }
        
        if let Ok(fortune) = self.inner.read() {
            if let Some(definition) = fortune.get_definition().get(&id_n) {
                // Some((fortune.lang.clone().unwrap_or("en_US".to_string()), definition.clone()))
                let i18n_code : &str = i18n_code.unwrap_or("0");
                let lang: &str = fortune.get_language("en_US");

                //查找翻译词条
                if let Some(i18n) = definition.i18n.get(i18n_code) {
                    if let Some((name, prompt_fmt)) = i18n.get(lang).or_else(|| i18n.get("en_US")) {
                        let mut prompts: Vec<&str> = Vec::new();
                        let mut prompt_fmt_pos = 0;

                        let re = regex::Regex::new(r"\{[a-zA-Z]+[a-zA-X0-9_]*\}").expect("Can't make regex");
                        for mat in re.find_iter(prompt_fmt) {
                            let cap_str = mat.as_str();
                            let name = cap_str.get(1..cap_str.len() - 1).unwrap();

                            prompts.push(prompt_fmt.get(prompt_fmt_pos..mat.start()).unwrap());
                            prompts.push(loc_param.get(name).map(|s| s.as_str()).unwrap_or(""));
                            prompt_fmt_pos = mat.end();
                        }
                        prompts.push(prompt_fmt.get(prompt_fmt_pos..).unwrap_or(""));


                        let mut param_prompts: Vec<String> = Vec::new();
                        let mut param_msgs: Vec<String> = Vec::new();

                        if let Some(param) = &definition.param {
                            for (param_code, param_i18n) in param {
                                if let Some(param_val_str) = loc_param.get(param_code.as_str()) {
                                    if let Some(param_name_i18n) = param_i18n.get(lang) {
                                        param_prompts.push(format!("({} : {})", param_name_i18n, param_val_str));
                                    }
                                    param_msgs.push(format!("({} : {})", param_code, param_val_str));
                                }
                            }
                        }

                        let prompt_str = prompts.join("");
                        return AnError::new(id_n.into(), definition.code.as_str(), name, loc, (prompt_str.as_str(), prompt_str.as_str()))
                            .put_msg_to(loc_msg)
                            .put_param_msgs_to(param_msgs.iter())
                            .put_param_prompts_to(param_prompts.iter());
                    }
                }
            }
        }

        AnError::new(id_n.into(), "FAILED", "Failed", loc, (loc_msg, loc_msg))
    }

    pub fn change_language(&mut self, lang:&str) -> bool {
        match self.inner.write() {
            Ok(mut m) => {
                m.change_language(lang);
                true
            },

            Err(_) => false
        }
    }

    pub fn init(&mut self, configuration_file:&str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // let mut rdr = csv::Reader::from_path(csv_file)?;
        // let _header = rdr.headers()?;
        // for result in rdr.deserialize() {
        //     let record: AnError = result?;
        //     self.put(vec![record]);
        // }

        let mut src_configurer: ErrorConfigurer = toml::from_str(BASIC_ERROR_DEFAULT).unwrap();
        
        if let Ok(mut file) = File::open(configuration_file) {
            let mut content_str = String::new();
            match file.read_to_string(&mut content_str) {
                Ok(s) => s,
                Err(e) => panic!("Read file failed: {}", e),
            };

            let configurer: ErrorConfigurer = toml::from_str(&content_str).unwrap();
            self.merge_configurer(&mut src_configurer, configurer);
        };

        match self.inner.write() {
            Ok(mut m) => {
                for definition in src_configurer.definitions {
                    m.get_definition_mut().insert(definition.id, definition);
                }
            },

            Err(e) => { panic!("Cant' write: {}", e); }
        }

        // let rv = if let Ok(mut file) = std::fs::File::create("1.txt") {
        //     self.write_to(&mut file)
        // } else {
        //     self.write_to(&mut std::io::stdout())
        // };
        // println!("{:?}", rv);

        Ok(())
    }
}