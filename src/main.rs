use std::fmt;
use std::collections::HashMap;

type ValCtx = HashMap<&'static str, LispVal>;
type FnCtx = HashMap<&'static str, LispVal>;

#[allow(dead_code)]
struct EnvCtx{
    env: ValCtx,
    fenv: FnCtx,
}

type Eval<T> = fn(EnvCtx) -> T;

type IFunc = fn(Vec<LispVal>) -> Eval<LispVal>;

#[allow(dead_code)]
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
                vals.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
            }),
            LispVal::Func(_) => write!(f, "(internal function)"),
            LispVal::Lambda(_, _) => write!(f, "(lambda function)"),
        }
    }
}

fn main() {
    use LispVal as lv;
    let vals = lv::List(vec![
        lv::Atom("fuck"),
        lv::Number(10),
        lv::Bool(true),
        lv::Bool(false),
        lv::String("foo"),
        lv::Nil,
        lv::List(vec![
            lv::Atom("bar"),
        ]),
    ]);
    println!("{}", vals)
}
