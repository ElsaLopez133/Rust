pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {


    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut proteins = Vec::new();
        for chunks in rna.as_bytes().chunks(3) {

        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
}



