use std::{collections::HashMap};

use sexpr_ir::values::{Handle, Symbol};

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

#[derive(Debug)]
pub struct Apply {
	pub callee: Expr,
	pub calls: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Lambda {
	pub name: Handle<Symbol>,
	pub body: Expr,
	pub catch_variable_table: HashMap<Handle<Symbol>, Expr>,
}

impl Lambda {
	pub fn new(name: &Handle<Symbol>, body: &Expr) -> Self {
		Lambda {
		    name: name.clone(),
		    body: body.clone(),
		    catch_variable_table: HashMap::new(),
		}
	}
}