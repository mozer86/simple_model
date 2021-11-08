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

use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    SimulationState,
    SimpleModel,
    Space,
    Surface,
    Fenestration,
    Luminaire,    
    hvac::{ 
        HVAC, 
        ElectricHeater, 
        IdealHeaterCooler        
    }
};






/// Registers the functions used to operate the building
pub fn register_control_api(engine : &mut rhai::Engine, model: &Rc<SimpleModel>, state: &Rc<RefCell<SimulationState>>, research_mode: bool){

    Space::register_api(engine, model, state, research_mode);
    Surface::register_api(engine, model, state, research_mode);
    Fenestration::register_api(engine, model, state, research_mode);
    Luminaire::register_api(engine, model, state, research_mode);

    // engine.register_type_with_name::<std::rc::Rc<dyn HVAC>>("HVAC");
    ElectricHeater::register_api(engine, model, state, research_mode);
    IdealHeaterCooler::register_api(engine, model, state, research_mode);
    

    let new_mod = std::rc::Rc::clone(model);  
    // let new_state = std::rc::Rc::clone(state);      
    engine.register_result_fn("hvac", move |name: &str | {
        for s in new_mod.hvacs.iter(){  
            match s {
                HVAC::ElectricHeater(s) =>{
                    if s.name == name {
                        let d = rhai::Dynamic::from(Rc::clone(s));
                        return Ok(d)
                    }
                },
                HVAC::IdealHeaterCooler(s)=>{
                    if s.name == name {
                        let d = rhai::Dynamic::from(Rc::clone(s));
                        return Ok(d)
                    }
                }
            }                      
        }
        return Err(format!("Could not find hvac '{}'", name).into());
    });

    

}