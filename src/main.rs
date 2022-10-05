fn main() {
    let path = std::env::args().nth(1).expect("entrypoint specified");
    let source_code = std::fs::read_to_string(&path).expect("entrypoint loaded");

    println!("{}", source_code);
}
