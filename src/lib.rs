mod ast;
mod parser;

#[cfg(test)]
mod tests {
    use super::parser::ExpParser;

    #[test]
    fn it_works() {
        println!("{:#?}", ExpParser::new().parse("{ foo = {} }"))
    }
}
