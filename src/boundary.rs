#[derive(Copy, Clone)]
pub enum Boundary {
    None,
    Ground,
    Space(usize),
}
