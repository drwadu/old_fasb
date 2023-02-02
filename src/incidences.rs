use crate::navigator::Navigator;

/// Incidence structure
pub enum Incidences {
    /// I: F x A -> {0,1}
    /// f I a <=> a in F^f or f == a
    FacetAtomCover,
    /// I: F x A -> {0,1}
    /// f I a <=> a in F^f
    FacetAtom,
}

pub(crate) trait X {
    fn exact_covers<T>(&mut self) -> Vec<Vec<T>>
    where
        T: std::fmt::Debug;
    fn extension(&self, nav: &mut Navigator);
}
impl X for Incidences {
    fn exact_covers<T>(&mut self) -> Vec<Vec<T>>
    where
        T: std::fmt::Debug,
    {
        todo!()
    }
    fn extension(&self, nav: &mut Navigator) {
        match self {
            Self::FacetAtomCover => {
                let fs = nav.current_facets.0.clone();

                let mut m = vec![];
                //fs.iter().for_each(|s| print!("{} ", s.to_string().unwrap()));
                for f in &fs {
                    let lit = *unsafe { nav.literals.get(f).unwrap_unchecked() };
                    let ffs = nav.inclusive_facets(&[lit]).0;
                    let v = fs
                        .iter()
                        .map(|x| match ffs.contains(x) || x == f {
                            true => 1,
                            _ => 0,
                        })
                        .collect::<Vec<u8>>();
                    v.iter().for_each(|i| print!("{:?} ", i));
                    print!("{}", f.to_string().unwrap());
                    println!();
                    m.push(v);
                    let ffs = nav.inclusive_facets(&[lit.negate()]).0;
                    let v = fs
                        .iter()
                        .map(|x| match ffs.contains(x) {
                            true => 1,
                            _ => 0,
                        })
                        .collect::<Vec<u8>>();
                    v.iter().for_each(|i| print!("{:?} ", i));
                    print!("-{} ", f.to_string().unwrap());
                    println!();
                    m.push(v)
                }
            }
            _ => println!(),
        }
    }
}

#[derive(Debug)]
pub struct Ctx {
    structure: Vec<Vec<u8>>,
    ordered: Vec<clingo::Symbol>,
}
impl std::fmt::Display for Ctx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ordered.iter().for_each(|s| {
            unsafe { write!(f, "{:?} ", s).unwrap_unchecked() };
        });
        Ok(())
    }
}
impl Ctx {
    pub(crate) fn new(nav: &mut Navigator) -> Self {
        let fs = nav.current_facets.0.clone();

        let mut m = vec![];
        for f in &fs {
            let ffs = nav
                .inclusive_facets(&[*unsafe { nav.literals.get(f).unwrap_unchecked() }])
                .0;
            let v = fs
                .iter()
                .map(|x| match ffs.contains(x) {
                    true => 1,
                    _ => 0,
                })
                .collect::<Vec<u8>>();
            m.push(v)
        }

        Self {
            structure: m,
            ordered: fs,
        }
    }

    /// Computes all exact covers of facet-incidence-matrix
    //    pub(crate) exact_cover(&mut self) {
    //        let com = self.components();
    //        let cc = unsafe {
    //            self.consequences(crate::navigator::EnumMode::Cautious, &[])
    //                .unwrap_unchecked()
    //        };
    //        let sorted_com = com
    //            .0
    //            .iter()
    //            .filter(|(cover, (_, _))| *cover != &cc)
    //            .collect::<Vec<_>>();
    //        // println!("sorted_com: {:?}", sorted_com);
    //
    //        let mut encoding = "".to_owned();
    //        let mut j = 0;
    //        sorted_com
    //            .iter()
    //            .filter(|(cover, (_, _))| cover.to_hashset() != cc.to_hashset())
    //            .enumerate()
    //            .for_each(|(i, (cover, (_, content)))| {
    //                cover.iter().filter(|a| !cc.contains(a)).for_each(|a| {
    //                    let s = unsafe { a.to_string().unwrap_unchecked() };
    //                    encoding = format!(
    //                        "{}\ncov({}) :- c({:?}).\n:- not cov({}).",
    //                        encoding, s, i, s,
    //                    )
    //                });
    //                content.iter().filter(|a| !cc.contains(a)).for_each(|a| {
    //                    encoding = format!(
    //                        "{}\ncon({},{:?}) :- c({:?}).",
    //                        encoding,
    //                        unsafe { a.to_string().unwrap_unchecked() },
    //                        i,
    //                        i
    //                    );
    //                });
    //
    //                j += 1;
    //            });
    //        // choose at least 2 components
    //        encoding = format!("{}\n2 {{c(X) : X=0..{:?}}}.", encoding, j - 1);
    //
    //        // project on components
    //        encoding = format!("{}\n#show c/1.", encoding);
    //        println!("{}", encoding);
    //    }
    //

    pub(crate) fn structure(&self, nav: &Navigator) {
        //let mut fs = nav
        //    .current_facets
        //    .iter()
        //    .map(|f| unsafe { f.to_string().unwrap_unchecked() })
        //    .collect::<Vec<_>>();
        //fs.sort();
        //dbg!(fs);
        for v in &self.structure {
            v.iter().for_each(|i| print!("{:?} ", i));
            println!();
        }
    }
    pub(crate) fn structure_sorted_by_sum(&self, nav: &Navigator) {
        let mut s = self
            .structure
            .iter()
            .map(|v| v.iter().map(|i| *i as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        s.sort_by(|a, b| {
            a.iter()
                .sum::<usize>()
                .partial_cmp(&b.iter().sum::<_>())
                .unwrap()
        });
        for v in &self.structure {
            v.iter().for_each(|i| print!("{:?} ", i));
            println!();
        }
    }
}
