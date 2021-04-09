mod structs;
mod parser;

use std::io::{Write, stdin, stdout};

use parser::{repl_parse};


fn main() -> ! {
    loop {
        // read
        stdout().write_all("λ ".as_bytes()).unwrap();
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        // parse
        let res = repl_parse(&input).unwrap();
        println!("res: {:?}", res);
    }
}
