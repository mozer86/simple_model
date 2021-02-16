use simulation_state::simulation_state::SimulationState;
use simulation_state::simulation_state_element::SimulationStateElement;

use crate::object_trait::ObjectTrait;


#[derive(Copy,Clone)]
pub enum HeatingCoolingKind {
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of 
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler,

    /// An electric heater, it can only 
    /// heat... does not cool.
    ElectricHeating,
}

pub struct HeaterCooler{
    
    /// The name of the system
    name: String,    

    /// The position of the system in its
    /// containing Array (this is not used for now, as 
    /// only one HeaterCooler is allowed per space)
    index: usize,

    /// The kind of heater utilized
    kind: HeatingCoolingKind,

    /// Max heating power
    max_heating_power : Option<f64>,

    /// Max cooling power
    max_cooling_power : Option<f64>,

    /// The index of the state of the heater in the 
    /// State array
    state_index: usize
}


impl ObjectTrait for HeaterCooler {
    
    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->String{
        format!("Heater/Cooler::{}",self.sub_class_name())
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->Result<(),String>{        
        // We need at least one value (each module using these systems
        // will check correctness, probably)
        if self.max_cooling_power.is_some() || self.max_heating_power.is_some(){
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }


}

impl HeaterCooler {

    pub fn new(state: &mut SimulationState, name: String, space_index: usize, kind: HeatingCoolingKind) -> Self {
        // Push this to state.        
        let state_index = state.push(
            // off by default,
            SimulationStateElement::SpaceHeatingCoolingPowerConsumption(space_index,0.0)
        );


        Self {
            name: name,
            index: 0,
            kind: kind,
            state_index: state_index,
            max_cooling_power: None,
            max_heating_power: None,
        }
    }
    
    pub fn state_index(&self)->usize{
        self.state_index
    }
    
    pub fn set_max_heating_power(&mut self, p: f64){
        self.max_heating_power = Some(p);
    }

    pub fn max_heating_power(&self)->Option<f64>{
        self.max_heating_power
    }
    
    pub fn set_max_cooling_power(&mut self, p: f64){
        self.max_cooling_power = Some(p);
    }

    pub fn max_cooling_power(&self)->Option<f64>{
        self.max_cooling_power
    }


    fn sub_class_name(&self)->&str{
        match self.kind {
            HeatingCoolingKind::IdealHeaterCooler => "IdealHeaterCooler",
            HeatingCoolingKind::ElectricHeating => "ElectricHeater",
        }
    }

    pub fn can_heat(&self)->bool{
        match self.kind{
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => true,
        }
    }

    pub fn can_cool(&self)->bool{
        match self.kind{
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => false,
        }
    }

    pub fn get_kind(&self)->HeatingCoolingKind{
        self.kind
    }
}