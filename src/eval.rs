
use sexpr_ir::values::Symbol;

use crate::structs::{Bind, Expr, GlobalEnv, Handle, Lambda, LocalEnv, Unit};

type EvalError = ();

type EvalResult = Result<Expr, EvalError>;

fn find_name<'a>(
	global_env: &'a GlobalEnv,
	local_env: &'a LocalEnv,
	k: &Handle<Symbol>) -> Option<&'a Expr> {
	if let Some(v) = local_env.get(k) {
		Some(v)
	} else {
		global_env.get(k)
	}
}


/* fn lazy_find_name(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	k: &Handle<Symbol>) -> Expr {
		find_name(global_env, local_env, &k)
		.map_or(Expr::Symbol(k.clone()), Expr::clone)
} */

fn funcall(global_env: &GlobalEnv,
	local_env: &LocalEnv,
	callee: &Lambda,
	prarm: &Expr) -> EvalResult {
		let mut env = callee.catch_variable_table.clone();
		let prarm = eval_expr(global_env, local_env, prarm)?;
		env.insert(
			callee.name.clone(),
			prarm);
		// eval_expr(global_env, local_env, prarm)?
			eval_expr(global_env, &env, &callee.body)
}

fn apply(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	callee: &Expr,
	prarm: &Expr) -> EvalResult {
	match callee {
		Expr::Lambda(l) => {
			funcall(global_env, local_env, l, prarm)
		}
	    Expr::Apply(a) => {
			let callee = apply(global_env, local_env, &a.callee, &a.prarm)?;
			apply(global_env, local_env, &callee, prarm)
		},
	    Expr::Symbol(k) => {
			find_name(global_env, local_env, &k)
				.map_or(
					Err(()),
				|callee| apply(global_env, local_env, callee, prarm))
		}
	}
}

pub fn eval_expr(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	code: &Expr) -> EvalResult {
	match code {
	    Expr::Apply(a) => apply(global_env, local_env, &a.callee,&a.prarm),
	    Expr::Lambda(v) => {
			let v = Lambda {
			    name: v.name.clone(),
			    body: v.body.clone(),
			    catch_variable_table: local_env.clone(),
			};
			Ok(Expr::Lambda(Handle::new(v)))
		},
	    Expr::Symbol(k) => {
			let r = find_name(global_env, local_env, &k)
				.map(Expr::clone)
				.or(Some(code.clone()))
				.unwrap();
			Ok(r)
		}
	}
}

pub fn eval(
	global_env: &mut GlobalEnv,
	local_env: &LocalEnv,
	code: &Unit) -> EvalResult {
	match code {
		Unit::Bind(Bind(k, v)) => {
			let v = eval_expr(global_env, local_env, v)?;
			global_env.insert(k.clone(), v.clone());
			Ok(v)
		}
		Unit::Expr(v) => eval_expr(global_env, local_env, v)
	}
}