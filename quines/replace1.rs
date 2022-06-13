const S: &str = "const S: &str = ...;\n\nfn main() {\n    print!(\"{}\", S.to_owned().replacen(\"...\", &format!(\"{:?}\", S), 1));\n}\n";

fn main() {
    print!("{}", S.to_owned().replacen("...", &format!("{:?}", S), 1));
}
