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
    fn to_negative_symbol(&self) -> Symbol;
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
    fn to_negative_symbol(&self) -> Symbol {
        Symbol::create_id(&self.as_ref()[1..], false).expect("converting to Symbol failed.")
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
impl AsRef<[Symbol]> for Facets {
    fn as_ref(&self) -> &[Symbol] {
        &self.0
    }
}
impl std::fmt::Display for Facets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, facet) in self.0.iter().enumerate() {
            if i as usize % 6 == 0 {
                writeln!(f, "").expect("displaying facets failed.");
            }
            write!(f, "{} ", facet.repr()).expect("displaying facets failed.");
            write!(f, "~{} ", facet.repr()).expect("displaying facets failed.");
        }

        writeln!(f, "")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route(pub Vec<String>);
impl Route {
    pub fn activate(&mut self, facet: impl Into<String>) {
        self.0.push(facet.into())
    }
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
    pub fn deactivate_any(&mut self, facet: String) -> Vec<usize> {
        let mut poss = vec![];
        while let Some(pos) = self.0.iter().position(|f| *f == facet) {
            self.0.remove(pos);
            poss.push(pos)
        }

        poss
    }
    pub fn peek_step(&self, facet: impl Into<String>) -> Route {
        let mut route = self.clone();
        route.activate(facet.into());

        route
    }
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
            .map(|u| Symbol::create_id((u as char).to_string().as_ref(), true).ok())
            .flatten()
            .collect::<Vec<Symbol>>();
        let v1 = (97..123u8)
            .map(|u| Symbol::create_id((u as char).to_string().as_ref(), false).ok())
            .flatten()
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
}
