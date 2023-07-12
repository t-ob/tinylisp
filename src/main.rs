fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tokens = tinylisp::tokenize(&input);
        let ast = tinylisp::parse(&tokens).unwrap();
        println!("{ast:#?}");
    }
}
