extern crate regex;

fn main() {
    let s = "women {name} man {id} xfafa";
    // let mut s0 = s.to_string();
    let re = regex::Regex::new(r"\{[a-zA-Z]+[a-zA-X0-9_]*\}").unwrap();
    let mut v: Vec<&str> = Vec::new();
    let mut pos = 0;
    for mat in re.find_iter(s) {
        // println!("{:?}", mat);
        //会崩溃，因为UTF-8的长度不固定，该方法没有解决UTF-8替换的问题。
        // s0.replace_range(mat.range(), &String::from("{我}"));
        v.push(s.get(pos..mat.start()).unwrap());
        v.push("{我}");
        pos = mat.end();
    }
    v.push(s.get(pos..).unwrap_or(""));

    println!("{}", v.join(""));

    // println!("{}", re.replace_all(&s0, |caps: &regex::Captures| {
    //     for cap in caps.iter() {
    //         println!("{}", cap.unwrap().as_str());
    //     }
    //     // format!("{} {}", &caps[1], &caps[2])
    //     format!("abc def")
    // }));

    // let e = ErrorHolder::singleton().get(0x800);
    // let e = errfmt!(0x0, "abc");
    // println!("{:?}", e);
}