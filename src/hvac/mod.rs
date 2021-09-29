/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


pub mod ideal_heater_cooler;
pub mod electric_heater;


use crate::model::SimpleModel;
use crate::hvac::ideal_heater_cooler::IdealHeaterCooler;
use crate::hvac::electric_heater::ElectricHeater;
use crate::scanner::{Scanner, TokenType};
use building_state_macro::GroupInputOutput;
use std::any::Any;


/// A collection of elements heating and cooling systems
#[derive(Debug, GroupInputOutput)]
pub enum HVACKind{
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler,

    /// An electric heater, it can only
    /// heat.
    ElectricHeater
}



pub fn cast_hvac<T>(system: &dyn HVAC)->Result<&T,String>
where T: HVAC + 'static
{
    if let Some(h) = system.as_any().downcast_ref::<T>() {                    
        Ok(h)
    } else {
        Err(format!("Invalid casting HVAC type... found type is {:?}", system.kind()))
    }
}

pub fn cast_mut_hvac<T>(system: &mut dyn HVAC)->Result<&mut T,String>
where T: HVAC + 'static
{   
    let kind = system.kind();
    if let Some(h) = system.as_mut_any().downcast_mut::<T>() {                    
        Ok(h)
    } else {
        Err(format!("Invalid mut casting HVAC type... found type is {:?}", kind))
    }
}

/// Shared functions for all objects in the [`HVAC`] group
pub trait HVAC {
    /// Gets the class of [`HVAC`], represented by the  
    /// variant within the [`HVACKind`] enum
    fn kind(&self)->HVACKind;

    /// Checks whether this specific [`HVAC`] can heat
    fn can_heat(&self)->bool;

    /// Checks whether this specific [`HVAC`] can cool
    fn can_cool(&self)->bool;

    /// Casts the [`HVAC`] object into an `Any`, which allows then 
    /// downcasting to the different kinds of [`HVAC`] 
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;


    /// Sets the index of 
    fn set_index(&mut self, index: usize){
        match self.kind(){
            HVACKind::ElectricHeater=>{
                // cast_mut_hvac::<ElectricHeater>(&mut self).expect("When setting index of ElectricHeater HVAC").set_index(index);
                if let Some(h) = self.as_mut_any().downcast_mut::<ElectricHeater>() {                    
                    h.set_index(index);
                } else {
                    panic!("Invalid casting HVAC type... found type is {:?}", self.kind())
                }
            },
            HVACKind::IdealHeaterCooler=>{
                if let Some(h) = self.as_mut_any().downcast_mut::<IdealHeaterCooler>() {                    
                    h.set_index(index);
                } else {
                    panic!("Invalid casting HVAC type... found type is {:?}", self.kind())
                }
            }
        }
    }
}




/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::space::Space;
    use std::rc::Rc;

    #[test]
    fn test_hvac_from_bytes(){
        let mut building = SimpleModel::new("the building".to_string());
        let bytes = b"{
            name: \"the space\"
        }";
        let space = Space::from_bytes(bytes, &mut building).unwrap();
        assert_eq!(space.name, "the space".to_string());
        let space = building.add_space(space);

        let bytes = b" ::ElectricHeater {
            name : \"A heater\",            
            target_space: \"the space\"
        }
        ";

        let heater = HVACKind::from_bytes(bytes, &mut building).unwrap();
        match &heater.kind(){
            HVACKind::ElectricHeater =>{
                if let Some(h) = heater.as_any().downcast_ref::<ElectricHeater>() {                    
                    if let Ok(s) = h.target_space() {
                        assert!(Rc::ptr_eq(&s, &space));
                    }
                } else {
                    panic!("Not an electric heater...?")
                }
            },
            HVACKind::IdealHeaterCooler=>{
                panic!("Definitely NOT an electric heater....!")
            }
        }
    }
}
