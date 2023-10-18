mod lc_121;
mod lc_13;
mod lc_58;
mod lc_14;

fn main() {
    let mut _in: Vec<String> = Vec::new();
    _in.push("flower".to_string());
    _in.push("flow".to_string());
    _in.push("dog".to_string());
    println!("{}", lc_14::longest_common_prefix(_in));
}
