use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};
use std::rc::Rc;
use crate::space::Space;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;
use crate::hvac::{HVAC, HVACKind};
use crate::scanner::{Scanner,TokenType};
use crate::model::SimpleModel;
use std::any::Any;

#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
pub struct ElectricHeater {
    /// The name of the system
    pub name: String,
    
    /// The [`Space`] that this [`ElectricHeater`] heats and/or
    /// cools
    pub target_space: Rc<Space>,
    
    /// Max heating power
    max_heating_power: Option<f64>,

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

}

