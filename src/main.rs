use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum EvalError {
    NotImplemented,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EvalError::NotImplemented => write!(f, "{}", "NotImplemented"),
        }
    }
}

type EvalResult = Result<LispVal, EvalError>;

type ValCtx = HashMap<&'static str, LispVal>;
type FnCtx = HashMap<&'static str, IFunc>;

trait ApplyTrait {
    fn apply(&mut self, name: &'static str, args: &Vec<LispVal>) -> EvalResult;
}

impl ApplyTrait for EnvCtx {
    fn apply(&mut self, name: &'static str, args: &Vec<LispVal>) -> EvalResult {
        match self.fenv.get(name) {
            Some(ifunc) => ifunc(self, args),
            None => Err(EvalError::NotImplemented),
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

fn add(_ctx: &mut EnvCtx, args: &Vec<LispVal>) -> EvalResult {
    match args.as_slice() {
        [] => Ok(LispVal::Number(0)),
        [n @ LispVal::Number(_)] => Ok(n.clone()),
        [s @ LispVal::String(_)] => Ok(s.clone()),
        [_v, _rest @ ..] => Err(EvalError::NotImplemented),
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
        LispVal::List(lst) => match lst.as_slice() {
            [LispVal::Atom("quota"), val] => Ok(val.clone()),
            [LispVal::Atom(fn_name), args @ ..] => ctx.apply(fn_name, &args.to_vec()),
            _ => Err(EvalError::NotImplemented),
        },
        LispVal::Atom(_) => Err(EvalError::NotImplemented),
        LispVal::Func(_) => Err(EvalError::NotImplemented),
        LispVal::Lambda(_, _) => Err(EvalError::NotImplemented),
        _ => Ok(val.clone()),
    }
}

fn main() {
    use LispVal as lv;
    // let vals = lv::List(vec![
    //     lv::Atom("fuck"),
    //     lv::Number(10),
    //     lv::Bool(true),
    //     lv::Bool(false),
    //     lv::String("foo"),
    //     lv::Nil,
    //     lv::List(vec![lv::Atom("bar")]),
    // ]);
    // println!("{}", vals);

    let mut ctx = EnvCtx {
        env: ValCtx::new(),
        fenv: FnCtx::new(),
    };
    ctx.fenv.insert("+", add);
    println!("{}", eval(&mut ctx, &lv::String("fuck")).unwrap());
    println!("{}", eval(&mut ctx, &lv::Number(2333)).unwrap());
    println!(
        "{}",
        eval(
            &mut ctx,
            &lv::List(vec![lv::Atom("quota"), lv::Number(42),])
        )
        .unwrap()
    );
    println!(
        "{}",
        eval(&mut ctx, &lv::List(vec![lv::Atom("+"), lv::Number(10)])).unwrap()
    );
}
