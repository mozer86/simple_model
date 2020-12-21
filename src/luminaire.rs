use simulation_state::simulation_state::SimulationState;
use simulation_state::simulation_state_element::SimulationStateElement;

use crate::object_trait::ObjectTrait;


pub struct Luminaire{
    
    /// The name of the Luminaire
    name: String,

    /// The position of the Luminaire in its
    /// containing Array (this is not used for now, as 
    /// only one HeaterCooler is allowed per space)
    index: usize,

    /// The maximum power consumption
    max_power: Option<f64>,

    /// The index of the state of the luminaire
    /// in the State array 
    state_index: usize,
}


impl ObjectTrait for Luminaire {
    
    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->String{
        format!("Luminaire")
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->Result<(),String>{        
        // We need at least one value (each module using these systems
        // will check correctness, probably)
        if self.max_power.is_some() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }


}


impl Luminaire {

    pub fn new(state: &mut SimulationState, name: String, space_index: usize ) -> Self {
        // Push this to state.
        let state_index = state.len();
        state.push(
            // off by default,
            SimulationStateElement::SpaceLightingPowerConsumption(space_index,0.0)
        );

        Self {
            name: name,
            index: 0,            
            state_index: state_index,
            max_power: None,
        }
    }
    
    pub fn state_index(&self)->usize{
        self.state_index
    }
    
    pub fn set_max_power(&mut self, p: f64){
        self.max_power = Some(p);
    }

    pub fn get_max_power(&self)->Option<f64>{
        self.max_power
    }
    
}