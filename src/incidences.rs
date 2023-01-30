use crate::navigator::Navigator;

pub struct Ctx {
    structure: Vec<Vec<u8>>,
    ordered: Vec<clingo::Symbol>,
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
