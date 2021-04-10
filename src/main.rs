mod structs;
mod parser;
mod eval;

use std::{collections::HashMap, io::{Write, stdin, stdout}};

use eval::{eval};
use parser::{repl_parse};


fn main() -> ! {
	let mut global_env = HashMap::new();
	let local_env = HashMap::new();
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
		// eval
		let res = eval(&mut global_env, &local_env, &res);
		// let res = lazy_eval_expr(&mut global_env, &local_env, &res);
        res.iter().for_each(|x|println!("> {}", x));
    }
}
