use std::collections::HashMap;
use std::fmt;

enum EvalError {
    NotImplemented,
    UnboundVar(&'static str),
    NumArgs(u32, u32),
    TypeMismatch(&'static str, &'static str),
    NotFunction(&'static str),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EvalError::NotImplemented => write!(f, "{}", "NotImplemented"),
            EvalError::UnboundVar(var) => write!(f, "Error UnboundVar: {}", var),
            EvalError::NumArgs(want, got) => {
                write!(f, "Error Number args, want: {}, but got: {}", want, got)
            }
            EvalError::TypeMismatch(want, got) => {
                write!(f, "Error Type Mismatch, want: {}, got: {}", want, got)
            }
            EvalError::NotFunction(name) => write!(f, "Error Not Function: {}", name),
        }
    }
}

type EvalResult = Result<LispVal, EvalError>;

fn print_result(res: &EvalResult) {
    match res {
        Ok(val) => println!("{}", val),
        Err(err) => eprintln!("{}", err),
    }
}

type ValCtx = HashMap<&'static str, LispVal>;
type FnCtx = HashMap<&'static str, IFunc>;

trait ApplyTrait {
    fn apply(&mut self, name: &'static str, args: &Vec<LispVal>) -> EvalResult;
}

impl ApplyTrait for EnvCtx {
    fn apply(&mut self, name: &'static str, args: &Vec<LispVal>) -> EvalResult {
        match self.fenv.get(name) {
            Some(ifunc) => ifunc(self, args),
            None => Err(EvalError::NotFunction(name)),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct EnvCtx {
    env: ValCtx,
    fenv: FnCtx,
}

type IFunc = fn(&mut EnvCtx, &Vec<LispVal>) -> EvalResult;

fn add(ctx: &mut EnvCtx, args: &Vec<LispVal>) -> EvalResult {
    match args.as_slice() {
        [] => Ok(LispVal::Number(0)),
        [n @ LispVal::Number(_)] => Ok(n.clone()),
        [s @ LispVal::String(_)] => Ok(s.clone()),
        [v, rest @ ..] => {
            let x = eval(ctx, v)?;
            let y = add(ctx, &rest.to_vec())?;
            match (x, y) {
                (LispVal::Number(x), LispVal::Number(y)) => Ok(LispVal::Number(x + y)),
                // (LispVal::String(x), LispVal::String(y)) => Ok(LispVal::String(&[x, y].concat())),
                _ => Err(EvalError::TypeMismatch("number", "other")),
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
enum LispVal {
    Atom(&'static str),
    List(Vec<LispVal>),
    Number(i32),
    String(&'static str),
    Nil,
    Bool(bool),
    Func(IFunc),
    Lambda(IFunc, EnvCtx),
}

impl fmt::Display for LispVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LispVal::Atom(atom) => write!(f, "{}", atom),
            LispVal::Number(n) => write!(f, "{}", n),
            LispVal::String(s) => write!(f, "\"{}\"", s),
            LispVal::Bool(b) => match b {
                true => write!(f, "#t"),
                false => write!(f, "#f"),
            },
            LispVal::Nil => write!(f, "'()"),
            LispVal::List(vals) => write!(f, "[{}]", {
                vals.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            }),
            LispVal::Func(_) => write!(f, "(internal function)"),
            LispVal::Lambda(_, _) => write!(f, "(lambda function)"),
        }
    }
}

fn eval(ctx: &mut EnvCtx, val: &LispVal) -> EvalResult {
    match val {
        LispVal::List(lst) => eval_list(ctx, lst),
        LispVal::Atom(_) => Err(EvalError::NotImplemented),
        LispVal::Func(_) => Err(EvalError::NotImplemented),
        LispVal::Lambda(_, _) => Err(EvalError::NotImplemented),
        _ => Ok(val.clone()),
    }
}

fn eval_list(ctx: &mut EnvCtx, list: &[LispVal]) -> EvalResult {
    match list {
        [LispVal::Atom("quota"), val] => Ok(val.clone()),
        [LispVal::Atom(fn_name), args @ ..] => ctx.apply(fn_name, &args.to_vec()),
        _ => Err(EvalError::NotImplemented),
    }
}

fn main() {
    use LispVal as lv;

    let mut ctx = EnvCtx {
        env: ValCtx::new(),
        fenv: FnCtx::new(),
    };
    ctx.fenv.insert("+", add);

    print_result(&eval(
        &mut ctx,
        &lv::List(vec![lv::Atom("quota"), lv::Number(42)]),
    ));
    print_result(&eval(
        &mut ctx,
        &lv::List(vec![
            lv::Atom("+"),
            lv::Number(10),
            lv::Number(20),
            lv::Number(30),
            lv::List(vec![lv::Atom("+"), lv::Number(10), lv::Number(20)]),
        ]),
    ));
    print_result(&eval(
        &mut ctx,
        &lv::List(vec![lv::Atom("-"), lv::Number(1)]),
    ));
}
