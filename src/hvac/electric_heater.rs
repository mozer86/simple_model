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

use crate::Float;


use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};
use std::rc::Rc;
use crate::space::Space;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;
use crate::hvac::{HVAC, HVACKind};
use crate::model::SimpleModel;
use std::any::Any;

#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
pub struct ElectricHeater {
    /// The name of the system
    pub name: String,
    
    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// The [`Space`] that this [`ElectricHeater`] heats and/or
    /// cools
    target_space: Option<Rc<Space>>,
    
    /// Max heating power
    max_heating_power: Option<Float>,

    #[state]
    heating_cooling_consumption: StateElementField,
}

impl HVAC for ElectricHeater{
    
    fn kind(&self)->HVACKind{
        HVACKind::ElectricHeater
    }

    fn can_heat(&self)->bool{
        true
    }

    fn can_cool(&self)->bool{
        false
    }

    fn as_any(&self) -> &dyn Any{
        self
    }   

    fn as_mut_any(&mut self) -> &mut dyn Any{
        self
    }  

    

}

