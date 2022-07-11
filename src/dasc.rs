use rug::Integer;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default)]
pub struct Dasc {
    decl: HashMap<String, String>,
    pub(crate) expr: Vec<String>,
}
impl Dasc {
    pub fn new(src: String) -> Option<Self> {
        let mut split = src.split("#dasc_start");

        let code = split.nth(1).map(|s| s.to_owned());

        parse(code)
    }
    pub fn count(&self, facets: &[String]) -> Integer {
        let mut tmp = HashMap::<String, Integer>::new();
        let mut result = Integer::new();
        for token in &self.expr {
            match self.decl.get(token) {
                Some(expr_) => match expr_.contains('#') {
                    true => {
                        let pred = expr_.split('#').last().unwrap();
                        let mut pred_split = pred.split('(');
                        let name = pred_split.next().unwrap().trim();
                        let rest_s = pred_split.next().unwrap().replace(')', "");
                        // dbg!(&rest_s);
                        let args_to_read = rest_s
                            .split(',')
                            .enumerate()
                            .filter(|(_, c)| *c != "_")
                            .map(|(i, _)| i)
                            .collect::<Vec<_>>();
                        // dbg!(&name);
                        // dbg!(&args_to_read);
                        let fc = facets
                            .iter()
                            .filter(|p| p.starts_with(name))
                            .map(|p| {
                                p.replace(name, "")
                                    .chars()
                                    .enumerate()
                                    .filter(|(i, _)| args_to_read.contains(&(i - 1)))
                                    .map(|(_, c)| c)
                                    .collect::<String>()
                            })
                            .collect::<HashSet<String>>().len();
                            tmp.insert(
                                token.to_string(),
                                Integer::from(fc),
                            );
                            // dbg!(&tmp);
                            // dbg!(facets
                            // .iter()
                            // .filter(|p| p.starts_with(name)).collect::<Vec<_>>());
                    }
                    _ => {
                        // check for [
                        tmp.insert(
                            token.to_string(),
                            calc(expr_.to_string()).expect("calc failed."),
                        );
                    }
                },
                _ => {
                    let mut eval_expr = vec![];
                    self.expr.iter().rev().for_each(|part| {
                        match tmp.get(part) {
                            Some(val) => eval_expr.push(format!("{:?}", val)),
                            _ => eval_expr.push(part.to_string()),
                        }
                    });
                    // dbg!(&needed);
                    // let used = tmp.get(needed).unwrap();
                    // let evaluated = token.replace(needed, format!("{:?}", used).as_ref());
                    // dbg!(&evaluated);
                    // dbg!(&eval_expr);
                    result = calc(eval_expr.join(" ")).unwrap();
                },
            }
        }
        // let mut result: Option<Integer> = None;

        //Integer::new()
        result
    }
}

fn parse(code: Option<String>) -> Option<Dasc> {
    match code {
        Some(src) => {
            let mut lines = src
                .lines()
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.starts_with('%') && !s.is_empty())
                .collect::<Vec<_>>();
            let n_vars = lines.len() - 2;
            dbg!(&lines);
            dbg!(n_vars);

            if let Some(line) = &lines[..n_vars + 2].iter().find(|s| {
                (s.contains('%') && s.contains("<-")) || (s.contains('%') && !s.starts_with('%'))
            }) {
                println!("error: invalid dasc syntax.\n --> {:?}", line);
                std::process::exit(-1);
            }

            if unsafe { lines.get_unchecked(n_vars + 1) } != "#dasc_end" {
                println!(
                    "error: invalid dasc syntax.\n --> terminate dasc code with {:?}",
                    "#dasc_end"
                );
                std::process::exit(-1);
            }
            lines.pop();

            let ret = unsafe { lines.get_unchecked(n_vars) };
            if ret.contains("<-") {
                println!("error: invalid dasc syntax.\n --> no return declared");
                std::process::exit(-1);
            }
            let expr = lines.pop();

            let decl = {
                let mut hm = HashMap::<String, String>::new();
                lines.iter().for_each(|line| {
                    dbg!(&line);
                    let mut iter = line.split(" <- ");
                    hm.insert(
                        iter.next().unwrap().trim().to_string(),
                        iter.next().unwrap().trim().to_string(),
                    );
                });

                hm
            };

            Some(Dasc {
                decl,
                expr: expr
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .rev()
                    .collect::<Vec<_>>(),
            })
        }
        _ => None,
    }
}

fn calc(expr: String) -> Option<Integer> {
    let rev_expr = expr.chars().rev().collect::<String>();
    let iter = rev_expr.split_whitespace();
    let mut mem = vec![];
    for t in iter {
        let t = t.chars().rev().collect::<String>();
        // dbg!(&t);
        // dbg!(&mem);
        match t.bytes().all(|c| c.is_ascii_digit()) {
            true => mem.push(Integer::from(t.parse::<usize>().unwrap())),
            _ => match t.chars().next() {
                Some('(') => mem.push(calc(t.replace(')', "").to_owned()).unwrap()),
                Some('+') => mem.push(mem.iter().sum::<Integer>()),
                Some('*') => mem.push(mem.iter().product::<Integer>()),
                Some('^') => {
                    let base = mem.last().unwrap();
                    let mut result = mem.last().unwrap().clone();
                    let exp = mem.get(mem.len() - 2).unwrap();
                    for _ in 0..exp.to_u64().unwrap() - 1 {
                        result *= base;
                    }
                    mem.push(result);
                }
                Some('!') => {
                    let result = factorial(mem.last().unwrap().clone());
                    mem.push(result);
                }
                Some('/') => mem.push(Integer::from(
                    mem.last().unwrap() / mem.get(mem.len() - 2).unwrap(),
                )),
                _ => (),
            },
        }
    }

    mem.last().cloned()
}

fn factorial(num: Integer) -> Integer {
    match num.to_usize() {
        Some(0) | Some(1) => Integer::from(1usize),
        _ => factorial(num.clone() - 1) * num,
    }
}

/*
#[derive(Clone, Debug)]
pub struct Dasc {
    decl: HashMap<String, String>,
    pub total: Integer,
    expr: String,
}
impl Dasc {
    pub fn new(src: String) -> Option<Self> {
        let mut split = src.split("#dasc_start");

        let code = split.nth(1).map(|s| s.to_owned());

        parse(code)
    }
    pub fn count(&self, facets: utils::Facets) -> Integer {
        Integer::new()
    }
}

fn parse(code: Option<String>) -> Option<Dasc> {
    match code {
        Some(src) => {
            let mut lines = src
                .lines()
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.starts_with('%') && !s.is_empty())
                .collect::<Vec<_>>();
            let n_vars = lines.len() - 2;
            dbg!(&lines);
            dbg!(n_vars);

            if let Some(line) = &lines[..n_vars + 2].iter().find(|s| {
                (s.contains('%') && s.contains("<-")) || (s.contains('%') && !s.starts_with('%'))
            }) {
                println!("error: invalid dasc syntax.\n --> {:?}", line);
                std::process::exit(-1);
            }

            if unsafe { lines.get_unchecked(n_vars + 1) } != "#dasc_end" {
                println!(
                    "error: invalid dasc syntax.\n --> terminate dasc code with {:?}",
                    "#dasc_end"
                );
                std::process::exit(-1);
            }
            lines.pop();

            let ret = unsafe { lines.get_unchecked(n_vars) };
            if ret.contains("<-") {
                println!("error: invalid dasc syntax.\n --> no return declared");
                std::process::exit(-1);
            }
            let expr = lines.pop();

            let total = match lines
                .iter()
                .position(|line| line.starts_with("#total"))
                .map(|i| lines.remove(i))
                .and_then(|expr| calc(expr.to_string()))
            {
                Some(i) => i,
                _ => {
                    println!("error: provide #total.");
                    std::process::exit(-1);
                }
            };

            let mut decl = {
                let mut hm = HashMap::<String, String>::new();
                lines.iter().for_each(|line| {
                    dbg!(&line);
                    let mut iter = line.split(" <- ");
                    hm.insert(
                        iter.next().unwrap().trim().to_string(),
                        iter.next().unwrap().trim().to_string(),
                    );
                });

                hm
            };
            //let total = decl
            //    .remove("#total")
            //    .and_then(|expr| calc(expr.to_string()));

            Some(Dasc {
                decl,
                total,
                expr: expr.unwrap(),
            })
        }
        _ => None,
    }
}
*/
