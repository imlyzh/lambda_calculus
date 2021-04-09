use std::{cell::RefCell, collections::VecDeque};

use pest::{Parser, error::Error};
use pest::iterators::Pair;
use pest_derive::*;

use sexpr_ir::values::{Symbol};

use crate::structs::*;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LambdaCalculus {}

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

impl ParseFrom<Rule> for Unit {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
			Rule::bind => Unit::Bind(Bind::parse_from(
				pair.into_inner().next().unwrap())),
			Rule::expr => Unit::Expr(Expr::parse_from(
					pair.into_inner().next().unwrap())),
			_ => unreachable!()
		}
    }
}

impl ParseFrom<Rule> for Bind {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();
		let name = inner.next().unwrap();
		let expr = inner.next().unwrap();
		let name = Symbol::parse_from(name);
		let expr = Expr::parse_from(expr);
		Bind(Handle::new(name), expr)
    }
}

impl ParseFrom<Rule> for Expr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
			Rule::expr => Expr::parse_from(
				pair.into_inner().next().unwrap()),
			Rule::lambda => Expr::Lambda(Handle::new(Lambda::parse_from(pair))),
			Rule::apply => Expr::Apply(Handle::new(Apply::parse_from(pair))),
			Rule::symbol => Expr::Symbol(Handle::new(Symbol::parse_from(pair))),
			_ => unreachable!()
		}
    }
}

impl ParseFrom<Rule> for Apply {
    fn parse_from(pair: Pair<Rule>) -> Self {
		let mut inner = pair.into_inner();
		let callee = inner.next().unwrap();
		let callee = Expr::parse_from(callee);
		let calls = inner
			.flat_map(|x| x.into_inner())
			.map(Expr::parse_from)
			.collect();
		Apply { callee, calls }
    }
}

impl ParseFrom<Rule> for Lambda {
    fn parse_from(pair: Pair<Rule>) -> Self {
		let mut inner = pair.into_inner();
		let name = inner.next().unwrap();
		let body = inner.next().unwrap();
		Lambda::new(
			&Handle::new(Symbol::parse_from(name)),
			&Expr::parse_from(body)
		)
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        Symbol {
            id: sexpr_ir::utils::string_intern(pair.as_str()),
            line,
            colum,
            pos,
            scope: RefCell::new(VecDeque::new()),
        }
    }
}


pub fn repl_parse(input: &str) -> Result<Unit, Error<Rule>> {
    let mut pair = LambdaCalculus::parse(Rule::repl_unit, input)?;
	println!("out: {}", &pair);
	let pair = pair
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
		.into_inner()
        .next()
        .unwrap();
    Ok(Unit::parse_from(pair))
}