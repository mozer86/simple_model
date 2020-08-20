use std::rc::Rc;

pub struct Substance {
    name: String,
    thermal_conductivity: f64,
    heat_capacity: f64,
    density: f64
}

impl Substance {

    pub fn new(name:String, thermal_conductivity: f64, heat_capacity: f64, density: f64)->Rc<Self>{
        Rc::new(Substance{
            name: name,
            thermal_conductivity: thermal_conductivity,
            heat_capacity: heat_capacity,
            density: density
        })
    }

    pub fn name(&self)->String{
        self.name.clone()
    }

    pub fn thermal_conductivity(&self)->f64{
        self.thermal_conductivity
    }

    pub fn heat_capacity(&self)->f64{
        self.heat_capacity
    }

    pub fn density(&self)->f64{
        self.density
    }

    pub fn thermal_diffusivity(&self) -> f64 {
        return self.thermal_conductivity/(self.density*self.heat_capacity)
    }
}
