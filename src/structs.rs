use std::{collections::{HashMap, LinkedList}, rc::Rc};

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

#[derive(Debug, Clone)]
pub struct Apply {
	pub callee: Expr,
	pub calls: Vec<Expr>,
}

pub type Env = HashMap<Handle<Symbol>, Expr>;

pub type GlobalEnv = Env;

#[derive(Debug, Clone)]
pub struct LocalEnv (pub LinkedList<Env>);

impl LocalEnv {
	pub fn new() -> Self {
		LocalEnv(LinkedList::new())
	}

	pub fn get(&self, k: &Handle<Symbol>) -> Option<&Expr> {
		for table in &self.0 {
			if let Some(v) = table.get(k) {
				return Some(v);
			}
		}
		None
	}
}



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