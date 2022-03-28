pub mod svc {
    mod any;
    pub use any::{ErrDefs};
}

// pub mod errormgr {
//     pub mod crate;
//     pub use self::{AnError, ErrorHolder};
// }
mod mgr;
pub use mgr::{Result, AnError, ErrorHolder};

mod util;
pub use util::StrUtil;


#[macro_export]
macro_rules! make_err {
    ($errid: expr, $fmt: expr, $( $name:ident=$arg:expr ),*) => {
        {
            // let re = regex::Regex::new(r"\{[a-zA-Z]+[a-zA-X0-9_]*\}").expect("Can't make regex");
            // for mat in re.find_iter(prompt_fmt) {
            //     let cap_str = mat.as_str();
            //     let name = cap_str.get(1..cap_str.len() - 1).unwrap();

            //     prompts.push(prompt_fmt.get(prompt_fmt_pos..mat.start()).unwrap());
            //     prompts.push(loc_param.get(name).map(|s| s.as_str()).unwrap_or(""));
            //     prompt_fmt_pos = mat.end();
            // }
            // prompts.push(prompt_fmt.get(prompt_fmt_pos..).unwrap_or(""));
            let param: std::collections::HashMap<&str, String> = vec![$((stringify!($name), format!("{}", $arg))),*].into_iter().collect();
            let msg: String = crate::StrUtil::format_str($fmt, &param);
            
            // let msg = format!($fmt, $($name=$arg),*);
            
            let loc: (&str, u32) = (file!(), line!());

            crate::ErrorHolder::singleton().get($errid, None, loc, msg.as_str(), param)
        }
    };

    ($errid: expr) => {
        make_err!($errid, "", )
    };

    ($an_error: expr => $errid: expr) => {
        make_err!($errid, "", ).extend_to($an_error)
    };

    ($errid: expr, $msg: expr) => {
        make_err!($errid, $msg, )
    };

    ($an_error: expr => $errid: expr, $msg: expr) => {
        make_err!($errid, $msg, ).extend_to($an_error)
    };

    ($an_error: expr => $errid: expr, $fmt: expr, $($name:ident=$arg:tt),*) => {
        make_err!($errid, $fmt, $($name=$arg)*).extend_to($an_error)
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn load_resource() {
        crate::ErrorHolder::singleton().init("config/error.toml");
        crate::ErrorHolder::singleton().change_language("en_US");

        let e0 = make_err!(0x201u32, "This {parameter_name}'s is empty", parameter_name="name");
        println!("{}\n{}", e0, e0.prompt());

        let mut map = std::collections::HashMap::new();
        map.insert("num1", 1);
        map.insert("num2", 2);
        let v =  map.get("num1").unwrap() +  map.get("num2").unwrap();
        let e1 = make_err!(0x201u32, "{n1} + {n2} =  {v}", parameter_name="name", n1 = map.get("num1").unwrap(),  n2 = map.get("num2").unwrap(), v = v);
        println!("{}\n{}", e1, e1.prompt());

        let e2 = make_err!(0x201u32, "More parameter", a = 0, b = 2, c = 2, d = 3);
        println!("{}\n{}", e2, e2.prompt());

        let e3 = make_err!(e1 => 0x1u32, "请求未传");
        println!("{}\n{}", e3, e3.prompt());

        let e4 = make_err!(e3 => 0x1u32, "请求未传2");
        println!("{}\n{}", e4, e4.prompt());

        let e5 = make_err!(e4 => 0x1u32);
        println!("{}\n{}", e5, e5.prompt());

        let e6 = make_err!(0x1u32);
        println!("{}\n{}", e6, e6.prompt());
    }
}
