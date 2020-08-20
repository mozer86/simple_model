use std::rc::Rc;
use crate::substance::*;

pub struct Material {    
    substance: Rc<Substance>,
    thickness: f64
}

impl Material {

    pub fn new(substance: Rc<Substance>, thickness: f64)->Rc<Self>{
        Rc::new(Material{
            substance: substance,
            thickness: thickness,
        })
    }

    pub fn thickness(&self)->f64{
        self.thickness
    }

    pub fn substance(&self)->Rc<Substance>{
        Rc::clone(&self.substance)
    }
}


