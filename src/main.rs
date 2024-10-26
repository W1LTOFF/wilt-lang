fn main() {
    let code_src = String::from_utf8(std::fs::read("res/test.wilt").unwrap()).unwrap();
    println!("{:#?}", code_src);
}
