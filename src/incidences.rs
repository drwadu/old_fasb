use crate::navigator::Navigator;

pub(crate) type Matrix<T> = Vec<Vec<T>>;

#[allow(dead_code)]
/// Incidence structure
pub enum Incidences {
    /// I: F x A -> {0,1}
    /// f I a <=> a in BC^f
    Brave,
    /// I: F x A -> {0,1}
    /// f I a <=> a in CC^f
    Cautious,
    /// I: F x A -> {0,1}
    /// f I a <=> a in F^f
    Facet,
}

#[derive(Debug)]
pub struct Table {
    incidences: crate::dlx::Matrix,
    initial_facets: Vec<clingo::Symbol>,
}
impl Table {
    pub fn new(nav: &mut Navigator, incidence: Incidences) -> Self {
        let initial_facets = nav.current_facets.0.clone();
        let mut incidences = crate::dlx::Matrix::new(initial_facets.len());

        match incidence {
            Incidences::Brave => {
                for f in &initial_facets {
                    let l = *unsafe { nav.literals.get(f).unwrap_unchecked() };

                    let is = unsafe {
                        nav.consequences(crate::navigator::EnumMode::Brave, &[l])
                            .unwrap_unchecked()
                    };
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);


                    let is = unsafe {
                        nav.consequences(crate::navigator::EnumMode::Brave, &[l.negate()])
                            .unwrap_unchecked()
                    };
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);
                }
            }
            Incidences::Cautious => {
                for f in &initial_facets {
                    let l = *unsafe { nav.literals.get(f).unwrap_unchecked() };

                    let is = unsafe {
                        nav.consequences(crate::navigator::EnumMode::Cautious, &[l])
                            .unwrap_unchecked()
                    };
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);


                    let is = unsafe {
                        nav.consequences(crate::navigator::EnumMode::Cautious, &[l.negate()])
                            .unwrap_unchecked()
                    };
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);
                }
            }
            _ => {
                for f in &initial_facets {
                    let l = *unsafe { nav.literals.get(f).unwrap_unchecked() };

                    let is = nav.inclusive_facets(&[l]).0;
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);
                    for b in v {
                        if *b {print!("1 ")} else {print!("0 ")}
                    }
                    println!();

                    let is = nav.inclusive_facets(&[l.negate()]).0;
                    let v = &initial_facets
                        .iter()
                        .map(|x| is.contains(x))
                        .collect::<Vec<_>>();
                    incidences.add_row(v);
                    for b in v {
                        if *b {print!("1 ")} else {print!("0 ")}
                    }
                    println!();
                }
            }
        }

        Self {
            incidences,
            initial_facets,
        }
    }
    pub fn max_exact_cover(&self) -> Vec<Vec<usize>> {
        let ecs = crate::dlx::solve_all(self.incidences.clone());
        let (n, s) = (ecs.len(), ecs.iter().map(|v| v.len()).sum::<usize>());
        println!("c found {:?} covers", n);
        println!("c meansize={:?}", s as f32/n as f32);
        ecs
    }
}
/*
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ordered.iter().for_each(|s| {
            unsafe { write!(f, "{:?} ", s).unwrap_unchecked() };
        });
        Ok(())
    }
}
*/

/*
        let mut lp = "".to_owned();
        for (i, r) in self.incidences.iter().enumerate() {
            for (j, u) in r.iter().enumerate() {
                lp = format!("{}im({:?},{:?},{:?}). ", lp, i, j, u);
            }
            lp = format!("{}\n", lp);
        }
        lp = format!(
            "{}
            {{ in(R):im(R,_,_) }}.
            #const n_atoms={:?}.
            :- in(R), in(R'), im(R,A,1), im(R',A,1), R!=R'.
            :- #count {{ A : im(R,A,1), in(R) }} != n_atoms.
            #maximize {{ R : in(R) }}.
            #show in/1.",
            lp,
            self.incidences.len() / 2
        );
        println!("{}",lp);

        let mut ctl = clingo::Control::new(vec!["1".to_owned()]).unwrap();
        ctl.add("base", &[], &lp).unwrap();
        ctl.ground(&[clingo::Part::new("base", &[]).unwrap()])
            .unwrap();
        let ec = ctl
            .all_models()
            .unwrap()
            .last()
            .map(|model| model.symbols)
            .unwrap_or_default();
        println!("{:?}", ec);
        let facets = ec
            .iter()
            .map(|symbol| unsafe {
                self.initial_facets.get(
                    symbol.to_string().unwrap_unchecked().replace("in(", "")[..1]
                        .parse::<usize>()
                        .unwrap_unchecked(),
                )
            })
            .flatten()
            .collect::<Vec<_>>();
        println!("{:?}", facets);

        //println!("{}", lp);
        todo!()
*/
