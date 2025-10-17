#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Location {
    Stock,
    Waste,
    Tableau(usize),
    Foundation(usize),
}
