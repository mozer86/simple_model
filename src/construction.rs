use std::rc::Rc;
use crate::material::*;

pub struct Construction {
    name: String,
    layers: Vec< Rc<Material> >
}
    
impl Construction {

    pub fn new(name: String, layers: Vec<Rc<Material>>)->Rc<Self>{
        Rc::new(Construction{
            name: name,
            layers: layers,
        })
    }    

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn n_layers(&self)->usize{
        self.layers.len()
    }

    pub fn layer(&self,i:usize)->Result<Rc<Material>,String>{
        if i >= self.layers.len(){
            return Err(format!("Index out of bounds... trying to access layer {} while there are only {}", i, self.layers.len()));
        }

        Ok(Rc::clone(&self.layers[i]))
    }

    pub fn r_value(&self)->f64{
        let mut r = 0.0;

        for layer in &self.layers {
            r += layer.thickness()/layer.substance().thermal_conductivity();
        }

        return r;
    }
}