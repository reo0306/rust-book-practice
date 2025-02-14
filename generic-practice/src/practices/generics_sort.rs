pub trait Sorter {
    fn sorter(&self, values: &mut [i32]);
}

pub struct AscendingSorter;

impl Sorter for AscendingSorter {
    fn sorter(&self, values: &mut [i32]) {
        values.sort();
    }
}

pub struct DescendingSorter;

impl Sorter for DescendingSorter {
    fn sorter(&self, values: &mut [i32]) {
        values.sort();
        values.reverse();
    }
}

pub fn get_sorter(asceding: bool) -> Box<dyn Sorter> {
    if asceding {
        return Box::new(AscendingSorter);
    } else {
        return Box::new(DescendingSorter);
    }
}