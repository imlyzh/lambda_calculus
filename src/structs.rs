use std::{collections::HashMap, fmt::Display, rc::Rc};

use sexpr_ir::values::Symbol;

pub type Handle<T> = Rc<T>;

#[derive(Debug)]
pub enum Unit {
	Bind(Bind),
	Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Bind (pub Handle<Symbol>, pub Expr);

#[derive(Debug, Clone)]
pub enum Expr {
	Apply(Handle<Apply>),
	Lambda(Handle<Lambda>),
	Symbol(Handle<Symbol>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Apply(v) => v.fmt(f),
            Expr::Lambda(v) => v.fmt(f),
            Expr::Symbol(v) => v.fmt(f)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Apply {
	pub callee: Expr,
	pub prarm: Expr,
}

impl Display for Apply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.callee, self.prarm)
    }
}

pub type Env = HashMap<Handle<Symbol>, Expr>;

pub type GlobalEnv = Env;

pub type LocalEnv = Env;

#[derive(Debug, Clone)]
pub struct Lambda {
	pub name: Handle<Symbol>,
	pub body: Expr,
	pub catch_variable_table: LocalEnv,
}

impl Lambda {
	pub fn new(name: &Handle<Symbol>, body: &Expr) -> Self {
		Lambda {
		    name: name.clone(),
		    body: body.clone(),
		    catch_variable_table: LocalEnv::new(),
		}
	}
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let l =
		if self.catch_variable_table.len() == 0 {
			"".to_string()
		} else {
			let l: Vec<String> = self.catch_variable_table
			.iter()
			.map(|(k, v)| format!("{}:{}", k, v))
			.collect();
			let l = l.join(", ");
			format!("[{}] ", l)
		};
        write!(f, "{}{} -> {}", l, self.name, self.body)
    }
}