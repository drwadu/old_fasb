use std::collections::HashSet;

use clingo::Symbol;

pub trait ToHashSet<T> {
    fn to_hashset(&self) -> HashSet<T>;
    fn difference(&self, other: &[T]) -> Vec<T>;
}
impl<T> ToHashSet<T> for Vec<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    fn to_hashset(&self) -> HashSet<T> {
        self.iter().cloned().collect::<HashSet<_>>()
    }
    fn difference(&self, other: &[T]) -> Vec<T> {
        let x = self.to_hashset();
        let y = &other.to_vec().to_hashset();

        x.difference(y).cloned().collect::<Vec<_>>()
    }
}

pub trait Repr {
    fn repr(&self) -> String;
    fn exclusive_repr(&self) -> String;
}
impl Repr for Symbol {
    fn repr(&self) -> String {
        self.to_string()
            .expect("Symbol to String conversion failed.")
    }
    #[cfg(not(tarpaulin_include))]
    fn exclusive_repr(&self) -> String {
        format!(
            "~{}",
            self.to_string()
                .expect("Symbol to String conversion failed.")
        )
    }
}

pub trait ToSymbol<T> {
    fn symbol(&self) -> Symbol;
    fn as_negative_symbol(&self) -> Symbol;
}
impl<T: AsRef<str>> ToSymbol<T> for T {
    fn symbol(&self) -> Symbol {
        let s = self.as_ref();
        match s.starts_with('-') {
            true => Symbol::create_id(&s[1..], false).expect("converting to Symbol failed."),
            _ => Symbol::create_id(s, true).expect("converting to Symbol failed."),
        }
    }
    fn as_negative_symbol(&self) -> Symbol {
        Symbol::create_id(self.as_ref(), false).expect("converting to Symbol failed.")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facets(pub Vec<Symbol>);
impl<'a> Facets {
    pub fn to_strings(&'a self) -> impl Iterator<Item = String> + 'a {
        self.iter().map(|sym| sym.repr())
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Symbol> {
        self.0.iter()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
#[cfg(not(tarpaulin_include))]
impl AsRef<[Symbol]> for Facets {
    fn as_ref(&self) -> &[Symbol] {
        &self.0
    }
}
#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Facets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, facet) in self.0.iter().enumerate() {
            if i as usize % 6 == 0 {
                writeln!(f).expect("displaying facets failed.");
            }
            write!(f, "{} ", facet.repr()).expect("displaying facets failed.");
            write!(f, "~{} ", facet.repr()).expect("displaying facets failed.");
        }

        writeln!(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route(pub Vec<String>);
impl Route {
    pub fn activate(&mut self, facet: impl Into<String>) {
        self.0.push(facet.into())
    }
    #[cfg(not(tarpaulin_include))]
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.0.iter()
    }
    pub fn deactivate_first_by_position(&mut self, position: usize) {
        self.0.remove(position);
    }
    pub fn deactivate_first(&mut self, facet: String) -> Option<usize> {
        if let Some(pos) = self.0.iter().position(|f| *f == facet) {
            self.0.remove(pos);
            return Some(pos);
        }

        None
    }
    #[cfg(not(tarpaulin_include))]
    pub fn deactivate_any(&mut self, facet: String) -> Vec<usize> {
        let mut poss = vec![];
        while let Some(pos) = self.0.iter().position(|f| *f == facet) {
            self.0.remove(pos);
            poss.push(pos)
        }

        poss
    }
    #[cfg(not(tarpaulin_include))]
    pub fn peek_step(&self, facet: impl Into<String>) -> Route {
        let mut route = self.clone();
        route.activate(facet.into());

        route
    }
    #[cfg(not(tarpaulin_include))]
    pub fn peek_steps(&self, facets: impl Iterator<Item = impl Into<String>>) -> Route {
        let mut route = self.clone();
        facets.for_each(|f| route.activate(f.into()));

        route
    }
    pub fn contains(&self, facet: impl Into<String>) -> bool {
        self.0.contains(&facet.into())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "< ").expect("displaying route failed.");
        self.iter().for_each(|facet| {
            write!(f, "{} ", facet).expect("displaying route failed.");
        });
        write!(f, ">")
    }
}
impl From<Route> for String {
    fn from(route: Route) -> Self {
        route.iter().cloned().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use clingo::{ClingoError, Symbol};
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn repr() -> Result<(), ClingoError> {
        assert_eq!(Symbol::create_id("a", true)?.repr(), "a");
        assert_eq!(Symbol::create_id("a", false)?.repr(), "-a");

        Ok(())
    }
    #[test]
    fn exclusive_repr() -> Result<(), ClingoError> {
        assert_eq!(Symbol::create_id("a", true)?.exclusive_repr(), "~a");
        assert_eq!(Symbol::create_id("a", false)?.exclusive_repr(), "~-a");

        Ok(())
    }
    #[test]
    fn to_hashset() {
        let v0 = (97..123u8)
            .flat_map(|u| Symbol::create_id((u as char).to_string().as_ref(), true).ok())
            .collect::<Vec<Symbol>>();
        let v1 = (97..123u8)
            .flat_map(|u| Symbol::create_id((u as char).to_string().as_ref(), false).ok())
            .collect::<Vec<Symbol>>();

        assert_eq!(v0.difference(&v0), vec![]);
        assert_eq!(v0.difference(&v1).to_hashset(), v0.to_hashset());
    }

    #[test]
    fn activate() {
        let chars = (97u8..123)
            .map(|c| (c as char).to_string())
            .collect::<Vec<String>>();
        let mut route = Route(chars.iter().map(|s| s.to_owned()).collect());

        assert_eq!(route.len(), 26);

        assert!(route.contains("a"));
        route.activate("a".to_owned());
        assert!(route.contains("a"));
        assert_eq!(route.len(), 27);

        route.activate("ä".to_owned());
        assert!(route.contains("ä"));
        assert_eq!(route.len(), 28);
    }
    #[test]
    fn deactivate_contains_len() {
        let chars = (97u8..123)
            .map(|c| (c as char).to_string())
            .collect::<Vec<String>>();
        let mut route = Route(chars.iter().map(|s| s.to_owned()).collect());

        assert_eq!(route.len(), 26);

        assert!(!route.contains("ä"));
        let dae = route.deactivate_first("ä".to_owned());
        assert!(dae.is_none());
        assert!(!route.contains("ä"));

        assert!(route.contains("a"));
        let dae = route.deactivate_first("a".to_owned());
        assert!(dae.is_some());
        dae.unwrap();
        assert!(!route.contains("a"));

        assert!(route.contains("b"));
        route.deactivate_first_by_position(0);
        assert!(!route.contains("b"));

        let mut chars_clone = chars.clone();
        for _ in 0..3 {
            chars_clone.push("a".to_string())
        }
        route = Route(chars_clone.iter().map(|s| s.to_owned()).collect());

        assert_eq!(route.len(), 29);
        route.deactivate_any("a".to_owned());
        assert!(!route.contains("a"));
        assert_eq!(route.len(), 25);
    }
    #[test]
    fn peek_step() {
        let chars = (97u8..123)
            .map(|c| (c as char).to_string())
            .collect::<Vec<String>>();
        let route = Route(chars.iter().map(|s| s.to_owned()).collect());

        let peek_route = route.peek_step("ä");
        assert!(peek_route.contains("ä"));

        assert!(route != peek_route);
    }
    #[test]
    fn peek_steps() {
        let chars = (97u8..123)
            .map(|c| (c as char).to_string())
            .collect::<Vec<String>>();
        let route = Route(chars.iter().map(|s| s.to_owned()).collect());

        let peek_route = route.peek_steps(["ä", "ü", "ö"].iter().map(|s| s.to_owned()));
        assert!(peek_route.contains("ä"));
        assert!(peek_route.contains("ü"));
        assert!(peek_route.contains("ö"));

        assert!(route != peek_route);
    }
    #[test]
    fn string_from_route() {
        let chars = (97u8..123)
            .map(|c| (c as char).to_string())
            .collect::<Vec<String>>();
        let route = Route(chars.iter().map(|s| s.to_owned()).collect());

        let string_from_route = String::from(route);
        assert_eq!(
            string_from_route,
            chars
                .iter()
                .map(|s| s.chars().next().unwrap_or('\0') as char)
                .collect::<String>()
        );
    }

    #[test]
    #[cfg(not(tarpaulin_include))] // somehow causes unrecognized lines
    fn to_strings() {
        let facets = Facets(
            (97u8..123)
                .map(|c| (c as char).to_string().symbol())
                .collect::<Vec<clingo::Symbol>>(),
        );
        let facets_as_strings = facets.to_strings();
        assert_eq!(
            facets_as_strings.collect::<Vec<String>>(),
            (97u8..123)
                .map(|c| (c as char).to_string())
                .collect::<Vec<String>>()
        );
    }
    #[test]
    fn is_empty_len() {
        let facets = Facets(vec![]);
        assert!(facets.is_empty());
        assert_eq!(facets.len(), 0);
    }

    #[test]
    fn symbol() -> Result<(), ClingoError> {
        let random_constant = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..100))
            .map(char::from)
            .collect::<String>();

        let sym = Symbol::create_id(&random_constant, true)?;
        assert_eq!(random_constant.symbol(), sym);

        let sym = Symbol::create_id(&random_constant, false)?;
        assert_eq!(format!("-{}", random_constant).symbol(), sym);

        Ok(())
    }
    #[test]
    fn as_negative_symbol() -> Result<(), ClingoError> {
        let random_constant = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rand::thread_rng().gen_range(1..100))
            .map(char::from)
            .collect::<String>();

        let sym = Symbol::create_id(&random_constant, false)?;
        assert_eq!(random_constant.as_negative_symbol(), sym);

        Ok(())
    }
}
