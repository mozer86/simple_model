
use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};
use std::rc::Rc;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element:: StateElementField;
use crate::space::Space;
use crate::hvac::{HVAC, HVACKind};
use crate::scanner::{Scanner,TokenType};
use crate::model::SimpleModel;

use std::any::Any;

/// An ideal Heating and Cooling device, with a COP of 1.
/// 
/// asd
#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
pub struct IdealHeaterCooler {
    /// The name of the system
    pub name: String,
    
    /// The `Space`s that this [`IdealHeaterCooler`] heats and/or
    /// cools
    pub target_spaces: Vec<Rc<Space>>,
    
    /// Max heating power
    max_heating_power: Option<f64>,

    /// Max cooling power
    max_cooling_power: Option<f64>,

    #[state]
    heating_cooling_consumption: StateElementField,
}


impl HVAC for IdealHeaterCooler{
    
    fn kind(&self)->HVACKind{
        HVACKind::IdealHeaterCooler
    }

    fn can_heat(&self)->bool{
        true
    }

    fn can_cool(&self)->bool{
        true
    }

    fn as_any(&self) -> &dyn Any{
        self
    }   
}
