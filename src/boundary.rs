// use std::cell::RefCell;
// use std::rc::Rc;
// use crate::space::Space;

#[derive(Clone)]
pub enum Boundary {
    Ground,
    Space(usize),
}
