
use sexpr_ir::values::Symbol;

use crate::structs::{Apply, Bind, Expr, GlobalEnv, Handle, Lambda, LocalEnv, Unit};

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

fn apply_once(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	callee: &Expr,
	parame: &Expr) -> Option<Expr> {
		todo!()
	}

fn apply_multi(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	a: &Handle<Apply>) -> Expr {
	let mut callee = a.callee.clone();
	let calls = a.calls.clone();
	for i in calls {
		// callee = apply_once(global_env, local_env, &callee, &i);
		todo!()
	}
	todo!()
}

fn lazy_eval_expr(
	global_env: &GlobalEnv,
	local_env: &LocalEnv,
	code: &Expr) -> Expr {
	match code {
	    Expr::Apply(a) => apply_multi(global_env, local_env, a),
	    Expr::Lambda(v) => {
			let mut local_env = local_env.0.clone();
			let mut cvt = v.catch_variable_table.0.clone();
			cvt.append(&mut local_env);
			let v = Lambda {
			    name: v.name.clone(),
			    body: v.body.clone(),
			    catch_variable_table: LocalEnv(cvt),
			};
			Expr::Lambda(Handle::new(v))
		},
	    Expr::Symbol(_) => code.clone()
	}
}

pub fn lazy_eval(
	global_env: &mut GlobalEnv,
	local_env: &LocalEnv,
	code: &Unit) -> Expr {
	match code {
		Unit::Bind(Bind(k, v)) => {
			let v = lazy_eval_expr(global_env, local_env, v);
			global_env.insert(k.clone(), v.clone());
			v
		}
		Unit::Expr(v) => lazy_eval_expr(global_env, local_env, v)
	}
}