use core::fmt;
use std::{collections::HashMap, fmt::format, io, num::ParseFloatError, rc::Rc};

#[derive(Clone)]
enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
    Bool(bool),
    Lambda(RispLambda),
}

#[derive(Clone)]
struct RispLambda {
    params: Rc<RispExp>,
    body: Rc<RispExp>,
}

enum RispErr {
    Reason(String),
}

#[derive(Clone)]
struct RispEnv<'a> {
    data: HashMap<String, RispExp>,
    outer: Option<&'a RispEnv<'a>>,
}

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(RispErr::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

/// 读取一块序列。一个单独的括号内的序列（表达式）。
fn read_seq<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let mut res: Vec<RispExp> = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(RispErr::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((RispExp::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expect a number".to_string())),
    }
}

fn parse_atom(token: &str) -> RispExp {
    match token {
        "true" => RispExp::Bool(true),
        "false" => RispExp::Bool(false),
        _ => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => RispExp::Number(v),
                Err(_) => RispExp::Symbol(token.to_string().clone()),
            }
        }
    }
}

// 参考 https://stopachka.essay.dev/post/5/risp-in-rust-lisp#comparison-operators
macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = floats
                .first()
                .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            };
            Ok(RispExp::Bool(f(first, rest)))
        }
    }};
}

fn default_env<'a>() -> RispEnv<'a> {
    let mut data: HashMap<String, RispExp> = HashMap::new();
    data.insert(
        "+".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);
            Ok(RispExp::Number(sum))
        }),
    );
    data.insert(
        "-".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or(RispErr::Reason("expected at least one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(RispExp::Number(first - sum_of_rest))
        }),
    );
    // = 逻辑实现
    let f1 = |args: &[RispExp]| -> Result<RispExp, RispErr> {
        let floats = parse_list_of_floats(args)?;
        // 要想比较，需要有两个值
        if floats.len() != 2 {
            return Err(RispErr::Reason("expected two number".to_string()));
        }
        // 将第 0 个元素和第 1 个元素进行比较
        if floats.get(0).is_none() || floats.get(1).is_none() {
            return Err(RispErr::Reason("expected number".to_string()));
        }
        let is_ok = floats.get(0).unwrap().eq(floats.get(1).unwrap());
        Ok(RispExp::Bool(is_ok))
    };
    data.insert("=".to_string(), RispExp::Func(f1));

    // 以宏的方式实现可以参考 https://stopachka.essay.dev/post/5/risp-in-rust-lisp#comparison-operators
    data.insert(
        "=".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            // 要想比较，需要有两个值
            if floats.len() != 2 {
                return Err(RispErr::Reason("expected two number".to_string()));
            }
            // 校验这两个值必须存在
            if floats.get(0).is_none() || floats.get(1).is_none() {
                return Err(RispErr::Reason("expected number".to_string()));
            }
            let is_ok = floats.get(0).unwrap().eq(floats.get(1).unwrap());
            Ok(RispExp::Bool(is_ok))
        }),
    );

    data.insert(
        ">=".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            // 要想比较，需要有两个值
            if floats.len() != 2 {
                return Err(RispErr::Reason("expected two number".to_string()));
            }
            // 校验这两个值必须存在
            if floats.get(0).is_none() || floats.get(1).is_none() {
                return Err(RispErr::Reason("expected number".to_string()));
            }
            Ok(RispExp::Bool(
                floats.get(0).unwrap().gt(floats.get(1).unwrap()),
            ))
        }),
    );

    data.insert(
        ">".to_string(),
        RispExp::Func(ensure_tonicity!(|a, b| a > b)),
    );

    data.insert(
        "<".to_string(),
        RispExp::Func(ensure_tonicity!(|a, b| a < b)),
    );

    data.insert(
        "<=".to_string(),
        RispExp::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    RispEnv { data, outer: None }
}

/// 从环境中查找标识符的值。先在内部作用域查找，找不到再到外层作用域查找
fn env_get(key: &str, env: &RispEnv) -> Option<RispExp> {
    match env.data.get(key) {
        Some(exp) => Some(exp.clone()),
        None => match env.outer {
            Some(outer_env) => env_get(key, &outer_env),
            None => None,
        },
    }
}

fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(k) => env_get(&k, env)
            .ok_or(RispErr::Reason(format!("unexpected symbol k={}", k)))
            .map(|x| x.clone()),
        RispExp::Number(_a) => Ok(exp.clone()),
        RispExp::Bool(_a) => Ok(exp.clone()),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            // 优先匹配并处理“关键字”
            match eval_built_in_form(first_form, arg_forms, env) {
                Some(built_in_res) => built_in_res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        RispExp::Func(f) => {
                            let args_eval = arg_forms
                                .iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<RispExp>, RispErr>>();
                            f(&args_eval?)
                        }
                        RispExp::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params, arg_forms, env)?;
                            eval(&lambda.body, new_env)
                        },
                        _ => Err(RispErr::Reason("first form must be a function".to_string())),
                    }
                }
            }
        },
        _ => {
            Err(RispErr::Reason("not supported type.".to_string()))
        }
    }
}

fn eval_forms(args: &[RispExp], env: &mut RispEnv) -> Result<Vec<RispExp>, RispErr> {
    args.iter().map(|x| eval(x, env)).collect()
}

fn env_for_lambda<'a>(
    params: Rc<RispExp>,
    args: &[RispExp],
    outer_env: &'a mut RispEnv,
) -> Result<RispEnv<'a>, RispErr> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != args.len() {
        return Err(RispErr::Reason(format!(
            "expected {} params, got {}",
            ks.len(),
            args.len()
        )));
    }
    let vs = eval_forms(args, outer_env)?;
    let mut data: HashMap<String, RispExp> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }

    Ok(RispEnv {
        data,
        outer: Some(outer_env),
    })
}

fn parse_list_of_symbol_strings(params: Rc<RispExp>) -> Result<Vec<String>, RispErr> {
    let list = match params.as_ref() {
        RispExp::List(s) => Ok(s.clone()),
        _ => Err(RispErr::Reason(format!("expected params to be a list"))),
    }?;
    list.iter()
        .map(|x| match x {
            RispExp::Symbol(s) => Ok(s.clone()),
            _ => Err(RispErr::Reason(format!(
                "expected symbol in the argument list"
            ))),
        })
        .collect()
}

// 处理内置标识符
fn eval_built_in_form(
    exp: &RispExp,
    other_args: &[RispExp],
    env: &mut RispEnv,
) -> Option<Result<RispExp, RispErr>> {
    match exp {
        RispExp::Symbol(symbol) => match symbol.as_ref() {
            "if" => Some(eval_if_args(other_args, env)),
            "def" => Some(eval_def_args(other_args, env)),
            "lambda" => Some(eval_lambda_args(other_args)),
            _ => None,
        },
        _ => None,
    }
}

/// if 语法的实现。`(if test conseq alt)`
fn eval_if_args(args: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let test_form = args
        .first()
        .ok_or(RispErr::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        RispExp::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = args
                .get(form_idx)
                .ok_or(RispErr::Reason(format!("expected form idx={}", form_idx)))?;
            let res_eval = eval(res_form, env);
            res_eval
        }
        _ => Err(RispErr::Reason(format!(
            "unexpected test form='{}'",
            test_form.to_string()
        ))),
    }
}

/// def 语法的实现。`(define var exp)`
fn eval_def_args(args: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let var_exp = args
        .first()
        .ok_or(RispErr::Reason(format!("unexepceted string for var")))?;

    let val_res = args
        .get(1)
        .ok_or(RispErr::Reason(format!("expected second param.")))?;
    let evaled_val = eval(val_res, env)?;

    match var_exp {
        RispExp::Symbol(ref var_name) => {
            env.data.insert(var_name.clone(), evaled_val);
            Ok(var_exp.clone())
        }
        _ => Err(RispErr::Reason(format!("unexpected var name"))),
    }
}

/// lambda 语法的实现。语法：`(lambda (var...) exp)`，例如表达式：`(lambda (r) (* 3.14 (* r r)))`
/// 由三部分组成，第一部分是关键字 `lambda`，第二部分是变量列表 `(r)`；第三部分是函数（lambda）主体 `(* 3.14 (* r r))`
fn eval_lambda_args(args: &[RispExp]) -> Result<RispExp, RispErr> {
    let params = args
        .first()
        .ok_or(RispErr::Reason(format!("unexpected args form")))?;
    let body = args
        .get(1)
        .ok_or(RispErr::Reason(format!("unexpected second form")))?;
    if args.len() != 2 {
        return Err(RispErr::Reason(format!("lambda can only have two forms")));
    }
    Ok(RispExp::Lambda(RispLambda {
        params: Rc::new(params.clone()),
        body: Rc::new(body.clone()),
    }))
}

/// display for RispExp
impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Number(n) => n.to_string(),
            RispExp::Bool(b_val) => b_val.to_string(),
            RispExp::Lambda(_) => "Lambda {}".to_string(),
            RispExp::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("{}", xs.join(","))
            }
            RispExp::Func(_) => "Function {}".to_string(),
        };
        write!(f, "{}", str)
    }
}

fn parse_eval(expr: String, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

pub fn run_repl() {
    let env = &mut default_env();
    loop {
        println!("risp >");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("// 🙀 => {}", msg),
            },
        }
    }
}

/// 分离出切片的第一个元素
/// 参考 https://doc.rust-lang.org/std/primitive.slice.html#method.split_first
fn split_first<T>(a: &[T]) -> Option<(&T, &[T])> {
    match a {
        [head, tail @ ..] => Some((head, tail)),
        [] => None,
    }
}

/// 分离出切片的前两个元素
fn split_prev_two<T>(a: &[T]) -> Option<(&T, &T, &[T])> {
    match a {
        [first, second, tail @ ..] => Some((first, second, tail)),
        [] => None,
        &[_] => {
            // 匹配其他情况：只有一个元素和两个元素的情况
            // 如果原切片只有1-2 个元素，那么无法匹配出 (&T, &T, &T) 的情况，此时直接返回 None
            None
        }
    }
}

/// 测试枚举类型中的函数

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "(+ 10 5)";
        let res = tokenize(src.into());
        println!("{:?}", res);
        assert!(false);
    }

    enum EA {
        Func(fn(a: isize, b: isize) -> bool),
    }

    #[test]
    fn test_enum_func() {
        // 方式 1
        fn f1(a: isize, b: isize) -> bool {
            a > b
        }
        let a = EA::Func(f1);
        // 方式 2
        let c1 = |a, b| a > b;
        let a = EA::Func(c1);
        // 方式 3
        let a = EA::Func(get_fn_or_closure());
    }

    fn get_fn_or_closure() -> fn(isize, isize) -> bool {
        // 这种闭包形式的代码，如果没有捕获（变量需要捕获），则被推导为函数指针。
        // > 没有捕获的闭包和函数指针相同
        |a, b| a > b
    }
}
