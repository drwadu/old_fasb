#![allow(dead_code)]

use clingo::Symbol;
use pest::Parser;

#[derive(Parser)]
#[grammar = "atom.pest"]
struct AtomParser;

#[derive(Debug, Clone)]
pub struct Atom<'a>(pub &'a str);
impl<'a> Atom<'a> {
    fn symbol(&self, arg: &str) -> Symbol {
        let is_pos = !arg.starts_with('-');
        let s = match is_pos {
            true => arg,
            _ => &arg[1..],
        };

        let is_num = s.parse::<i32>().is_ok();

        match is_num {
            true => match is_pos {
                true => s
                    .parse::<i32>()
                    .map(Symbol::create_number)
                    .expect("parsing i32 failed."),
                _ => {
                    let uint = s.parse::<i32>().expect("parsing i32 failed.");

                    Symbol::create_number(-uint)
                }
            },
            _ => match s.starts_with('"') {
                true => Symbol::create_string(&s[1..s.len() - 1]).expect("parsing string failed."),
                _ => Symbol::create_id(s, is_pos).expect("parsing string failed."),
            },
        }
    }
    pub fn parse(&self, prefixes: &[char]) -> Option<Symbol> {
        let mut expr = self.0.trim();

        let is_pos = !prefixes.iter().any(|p| expr.starts_with(*p));

        expr = match is_pos {
            true => expr,
            _ => &expr[1..],
        };

        match AtomParser::parse(Rule::atom, expr) {
            Ok(atom) => {
                let mut ps = atom.flatten();

                let n = self
                    .0
                    .replace('(', "|")
                    .replace(',', "|")
                    .split('|')
                    .count();
                if ps.clone().skip(3).count() != n - 1 {
                    return None;
                }

                let name = ps.nth(1).map(|s| s.as_str())?;
                if !expr.contains('(') {
                    return Some(Symbol::create_id(name, is_pos).expect("creating Symbol failed."));
                }

                let arguments = ps
                    .skip(1)
                    .map(|arg| self.symbol(arg.as_str().replace(",", "").as_str()))
                    .into_iter()
                    .collect::<Vec<Symbol>>();

                Some(
                    Symbol::create_function(name, &arguments, is_pos)
                        .expect("creating Symbol failed."),
                )
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use clingo::{ClingoError, Symbol};
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn positive_constant() -> Result<(), ClingoError> {
        let random_constant = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..100))
            .map(char::from)
            .collect::<String>();
        // dbg!(random_constant.clone());

        let atom = Atom(random_constant.as_ref());
        match random_constant
            .chars()
            .next()
            .map(char::is_numeric)
            .unwrap_or(false)
            || random_constant
                .chars()
                .next()
                .map(char::is_uppercase)
                .unwrap_or(false)
        {
            true => assert!(atom.parse(&['~']).is_none()),
            _ => {
                assert!(atom.parse(&[]).is_some());
                assert_eq!(
                    Symbol::create_id(random_constant.as_ref(), true)?,
                    atom.parse(&[]).unwrap()
                );
            }
        }

        Ok(())
    }

    #[test]
    fn negative_constant() -> Result<(), ClingoError> {
        let random_constant = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..100))
            .map(char::from)
            .collect::<String>();
        // dbg!(random_constant.clone());
        let s = format!("~{}", random_constant.clone());

        let atom = Atom(s.as_ref());

        match random_constant
            .chars()
            .next()
            .map(char::is_numeric)
            .unwrap_or(false)
            || random_constant
                .chars()
                .next()
                .map(char::is_uppercase)
                .unwrap_or(false)
        {
            true => assert!(atom.parse(&['~']).is_none()),
            _ => {
                assert!(atom.parse(&['~']).is_some());
                assert_eq!(
                    Symbol::create_id(random_constant.as_ref(), false)?,
                    atom.parse(&['~']).unwrap()
                );
            }
        }

        Ok(())
    }

    #[test]
    fn arity_3() -> Result<(), ClingoError> {
        let random_name = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..3))
            .map(char::from)
            .collect::<String>();
        // dbg!(random_name.clone());

        let random_string = format!(
            "\"{}\"",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(rand::thread_rng().gen_range(1..10))
                .map(char::from)
                .collect::<String>()
        );
        // dbg!(random_string.clone());

        let random_constant = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..100))
            .map(char::from)
            .collect::<String>();
        // dbg!(random_constant.clone());

        let random_int = rand::thread_rng().gen::<i32>().to_string();
        // dbg!(random_int.clone());

        let p = format!(
            "{}({}, {}, {})",
            random_name, random_constant, random_string, random_int
        );
        dbg!(p.clone());

        let atom = Atom(p.as_ref());
        match (random_name
            .chars()
            .next()
            .map(char::is_numeric)
            .unwrap_or(false)
            || random_name
                .chars()
                .next()
                .map(char::is_uppercase)
                .unwrap_or(false))
            || (random_constant
                .chars()
                .next()
                .map(char::is_numeric)
                .unwrap_or(false)
                || random_constant
                    .chars()
                    .next()
                    .map(char::is_uppercase)
                    .unwrap_or(false))
        {
            true => assert!(atom.parse(&[]).is_none()),
            _ => {
                assert!(atom.parse(&[]).is_some());
                dbg!(atom.parse(&[]).unwrap().to_string().unwrap());
            }
        };

        Ok(())
    }
}
