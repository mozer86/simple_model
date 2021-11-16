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


mod ideal_heater_cooler;
mod electric_heater;


pub use crate::hvac::ideal_heater_cooler::IdealHeaterCooler;
pub use crate::hvac::electric_heater::ElectricHeater;

use crate::model::SimpleModel;
use building_state_macro::{
    SimpleGroupInputOutput,
    GroupSimpleRhaiAPI
};
use std::rc::Rc;


/// A collection of elements heating and cooling systems
#[derive( Clone, GroupSimpleRhaiAPI, SimpleGroupInputOutput)]
pub enum HVAC{
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler(Rc<IdealHeaterCooler>),

    /// An electric heater, it can only
    /// heat.
    ElectricHeater(Rc<ElectricHeater>)
}



use crate::simulation_state_element::SimulationStateElement;
use crate::simulation_state::SimulationStateHeader;

impl SimpleModel {

    /// Adds a [`HVAC`] to the [`SimpleModel`]
    pub fn add_hvac(&mut self, mut add : HVAC, state: &mut SimulationStateHeader ) -> HVAC {

        // Check the index of this object
        let obj_index = self.hvacs.len();
        match &mut add {
            HVAC::ElectricHeater(hvac)=>{
                let hvac = Rc::get_mut(hvac).expect("Could not borrow ElectricHeater as mut when adding");
                hvac.set_index(obj_index);
                let state_index = state.push( SimulationStateElement::HeatingCoolingPowerConsumption(obj_index), 0.);
                hvac.set_heating_cooling_consumption_index(state_index);                
            },
            HVAC::IdealHeaterCooler(hvac)=>{
                let hvac = Rc::get_mut(hvac).expect("Could not borrow IdealHeaterCooler as mut when adding");
                hvac.set_index(obj_index);
                let state_index = state.push( SimulationStateElement::HeatingCoolingPowerConsumption(obj_index), 0.);
                hvac.set_heating_cooling_consumption_index(state_index);                                               
            }
        }
                
        // Add to model, and return a reference                
        self.hvacs.push(add.clone());
        add
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
        let mut model = SimpleModel::new("the model".to_string());
        let bytes = b"{
            name: \"the space\"
        }";
        let space = Space::from_bytes(1, bytes, &mut model).unwrap();
        assert_eq!(space.name, "the space".to_string());
        let space = model.add_space(space);

        let bytes = b" ::ElectricHeater {
            name : \"A heater\",            
            target_space: \"the space\"
        }
        ";

        let heater = HVAC::from_bytes(1, bytes, &mut model);
        if let Ok(HVAC::ElectricHeater(h)) = &heater{
            if let Ok(s) = h.target_space() {
                assert!(Rc::ptr_eq(&s, &space));
            }            
            
        }else{
            panic!("Definitely NOT an electric heater....!")
        }
    }
}
