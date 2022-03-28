extern crate regex;

pub struct StrUtil{}

impl StrUtil {
    pub fn format_str(fmt:&str, param: &std::collections::HashMap<&str, String>) -> String {
        let mut pos = 0;
        let mut msgs = regex::Regex::new(r"\{[a-zA-Z]+[a-zA-X0-9_]*\}").expect("Can't make regex")
        .find_iter(fmt)
        .fold(Vec::new(), |mut v, mat| {
            v.push(&fmt[pos..mat.start()]);
            param.get(&fmt[mat.start() + 1..mat.end() - 1]).map(|s| v.push(s));
            pos = mat.end();
            v
        });
        msgs.push(&fmt[pos..]);
        msgs.join("")
    }
}