use crate::utils::ToHashSet;
use clingo::Symbol;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub(crate) struct Components(pub HashMap<Vec<Symbol>, (HashSet<String>, HashSet<Symbol>)>);

#[derive(Debug, Clone, Default)]
pub(crate) struct Interiors(pub HashMap<Vec<Symbol>, (HashSet<String>, HashSet<Symbol>)>);

#[derive(Debug, Clone, Default)]
pub(crate) struct Exteriors(HashMap<Vec<Symbol>, (HashSet<String>, HashSet<Symbol>)>);

pub(crate) trait AsnC {
    fn interiors(&mut self) -> Interiors;
    fn exteriors(&mut self) -> Exteriors;
    fn components(&mut self) -> Components;
    fn related_components(&mut self) -> Components;
}
impl AsnC for crate::navigator::Navigator {
    fn interiors(&mut self) -> Interiors {
        let mut hm = Interiors::default();
        let lits = self.literals.clone();
        self.inclusive_facets(&self.active_facets.clone())
            .iter()
            .for_each(|f| {
                let s = unsafe { f.to_string().unwrap_unchecked() };
                let l = unsafe { lits.get(f).unwrap_unchecked() };
                let bcs = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[*l])
                        .unwrap_unchecked()
                };
                let v =
                    hm.0.entry(unsafe {
                        self.consequences(crate::navigator::EnumMode::Cautious, &[*l])
                            .unwrap_unchecked()
                    })
                    .or_insert_with(|| (vec![s.clone()].to_hashset(), bcs.to_hashset()));
                v.0.insert(s.clone());
                v.1.extend(bcs);

                let bcs = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[l.negate()])
                        .unwrap_unchecked()
                };
                let v =
                    hm.0.entry(unsafe {
                        self.consequences(crate::navigator::EnumMode::Cautious, &[l.negate()])
                            .unwrap_unchecked()
                    })
                    .or_insert_with(|| (vec![format!("~{}", s)].to_hashset(), bcs.to_hashset()));
                v.0.insert(format!("~{}", s));
                v.1.extend(bcs);
            });
        hm
    }
    fn exteriors(&mut self) -> Exteriors {
        let mut hm = Exteriors::default();
        let lits = self.literals.clone();
        self.inclusive_facets(&self.active_facets.clone())
            .iter()
            .for_each(|f| {
                let s = unsafe { f.to_string().unwrap_unchecked() };
                let l = unsafe { lits.get(f).unwrap_unchecked() };
                let ccs = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[*l])
                        .unwrap_unchecked()
                };
                let v =
                    hm.0.entry(unsafe {
                        self.consequences(crate::navigator::EnumMode::Cautious, &[*l])
                            .unwrap_unchecked()
                    })
                    .or_insert_with(|| (vec![s.clone()].to_hashset(), ccs.to_hashset()));
                v.0.insert(s.clone());
                v.1.extend(ccs);

                let ccs = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[l.negate()])
                        .unwrap_unchecked()
                };
                let v =
                    hm.0.entry(unsafe {
                        self.consequences(crate::navigator::EnumMode::Cautious, &[l.negate()])
                            .unwrap_unchecked()
                    })
                    .or_insert_with(|| (vec![format!("~{}", s)].to_hashset(), ccs.to_hashset()));
                v.0.insert(format!("~{}", s));
                v.1.extend(ccs);
            });
        hm
    }
    fn components(&mut self) -> Components {
        let mut connected_components = Components::default();
        let lits = self.literals.clone();
        self.inclusive_facets(&self.active_facets.clone())
            .iter()
            .for_each(|facet| {
                let facet_string = unsafe { facet.to_string().unwrap_unchecked() };
                let literal = unsafe { lits.get(facet).unwrap_unchecked() };

                // inclusive facet
                let mut brave_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[*literal])
                        .unwrap_unchecked()
                };
                let mut cautious_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[*literal])
                        .unwrap_unchecked()
                };

                let mut cover = connected_components
                    .0
                    .entry(cautious_consequences)
                    .or_insert_with(|| {
                        (
                            vec![facet_string.clone()].to_hashset(),
                            brave_consequences.to_hashset(),
                        )
                    });
                cover.0.insert(facet_string.clone()); // constituting component
                cover.1.extend(brave_consequences); // collecting content

                // exclusive facet
                brave_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[literal.negate()])
                        .unwrap_unchecked()
                };
                cautious_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[literal.negate()])
                        .unwrap_unchecked()
                };
                cover = connected_components
                    .0
                    .entry(cautious_consequences)
                    .or_insert_with(|| {
                        (
                            vec![format!("~{}", facet_string)].to_hashset(),
                            brave_consequences.to_hashset(),
                        )
                    });
                cover.0.insert(format!("~{}", facet_string)); // constituting component
                cover.1.extend(brave_consequences); // collecting content
            });

        connected_components
    }
    fn related_components(&mut self) -> Components {
        let mut related_components = Components::default();
        let lits = self.literals.clone();
        self.inclusive_facets(&self.active_facets.clone())
            .iter()
            .for_each(|facet| {
                let facet_string = unsafe { facet.to_string().unwrap_unchecked() };
                let literal = unsafe { lits.get(facet).unwrap_unchecked() };

                // inclusive facet
                let mut brave_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[*literal])
                        .unwrap_unchecked()
                };
                let mut cautious_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[*literal])
                        .unwrap_unchecked()
                };

                let mut content = related_components
                    .0
                    .entry(brave_consequences)
                    .or_insert_with(|| {
                        (
                            vec![facet_string.clone()].to_hashset(),
                            cautious_consequences.to_hashset(),
                        )
                    });
                content.0.insert(facet_string.clone()); // constituting component
                content.1.extend(cautious_consequences); // collecting cover

                // exclusive facet
                brave_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Brave, &[literal.negate()])
                        .unwrap_unchecked()
                };
                cautious_consequences = unsafe {
                    self.consequences(crate::navigator::EnumMode::Cautious, &[literal.negate()])
                        .unwrap_unchecked()
                };
                content = related_components
                    .0
                    .entry(brave_consequences)
                    .or_insert_with(|| {
                        (
                            vec![format!("~{}", facet_string)].to_hashset(),
                            cautious_consequences.to_hashset(),
                        )
                    });
                content.0.insert(format!("~{}", facet_string)); // constituting component
                content.1.extend(cautious_consequences); // collecting cover
            });

        related_components
    }
}
