use core::fmt;
use std::{collections::HashMap, io, num::ParseFloatError};

#[derive(Clone)]
enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>),
    Bool(bool),
}

enum RispErr {
    Reason(String),
}

#[derive(Clone)]
struct RispEnv {
    data: HashMap<String, RispExp>,
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

// assembly the cmp func
// fn get_cmp_func(cmp_func: impl Fn(&f64, &f64) -> bool) -> fn(&[RispExp]) -> Result<RispExp, RispErr> {
//     let f =  |args: &[RispExp]| -> Result<RispExp, RispErr> {
//         let floats = parse_list_of_floats(args)?;
//         let first = floats.first().ok_or(RispErr::Reason("expected at least one number".to_string()))?;
//         // 要想比较，需要有两个值
//         if floats.len() != 2 {
//             return Err(RispErr::Reason("expected two number".to_string()));
//         }
//         // 将第 0 个元素和第 1 个元素进行比较
//         if floats.get(0).is_none() || floats.get(1).is_none() {
//             return Err(RispErr::Reason("expected number".to_string()));
//         }
//         Ok(RispExp::Bool(cmp_func(floats.get(0).unwrap(), floats.get(1).unwrap())))
//     };
//     return f;
// }

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

fn default_env() -> RispEnv {
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
            // 将第 0 个元素和第 1 个元素进行比较
            if floats.get(0).is_none() || floats.get(1).is_none() {
                return Err(RispErr::Reason("expected number".to_string()));
            }
            let is_ok = floats.get(0).unwrap().eq(floats.get(1).unwrap());
            Ok(RispExp::Bool(is_ok))
        }),
    );
    data.insert(
        ">".to_string(),
        RispExp::Func(ensure_tonicity!(|a, b| a > b)),
    );

    data.insert(
        ">=".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            Ok(RispExp::Bool(
                floats.get(0).unwrap().gt(floats.get(1).unwrap()),
            ))
        }),
    );

    RispEnv { data }
}

fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(k) => env
            .data
            .get(k)
            .ok_or(RispErr::Reason(format!("unexpected symbol k={}", k)))
            .map(|x| x.clone()),
        RispExp::Number(_a) => Ok(exp.clone()),
        RispExp::Bool(_a) => Ok(exp.clone()),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(RispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                RispExp::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<RispExp>, RispErr>>();
                    f(&args_eval?)
                }
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExp::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

/// display for RispExp
impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Number(n) => n.to_string(),
            RispExp::Bool(b_val) => b_val.to_string(),
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
        let c1 = |a, b| {a > b};
        let a = EA::Func(c1);
        // 方式 3
        let a = EA::Func(get_fn_or_closure());
    }

    fn get_fn_or_closure() -> fn(isize, isize) -> bool {
        // 这种闭包形式的代码，如果没有捕获，则被推导为函数指针。
        // > 没有捕获的闭包和函数指针相同
        |a, b| a > b
    }
}
