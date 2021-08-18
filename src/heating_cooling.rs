use building_state_macro::BuildingObjectBehaviour;
// use crate::simulation_state::SimulationState;
// use crate::simulation_state_element::SimulationStateElement;


#[derive(Copy, Clone)]
pub enum HeatingCoolingKind {
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler,

    /// An electric heater, it can only
    /// heat... does not cool.
    ElectricHeating,
}

#[derive(Clone,BuildingObjectBehaviour)]
pub struct HeaterCooler {
    
    /// The name of the system
    pub name: String,
    
    /// The kind of heater utilized
    pub kind: HeatingCoolingKind,
    
    /// The position of the system in its
    /// containing Array (this is not used for now, as
    /// only one HeaterCooler is allowed per space)
    index: Option<usize>,

    /// Max heating power
    max_heating_power: Option<f64>,

    /// Max cooling power
    max_cooling_power: Option<f64>,

    #[state]
    heating_cooling_consumption: Option<usize>,
}



impl HeaterCooler {
    

    // fn sub_class_name(&self) -> &str {
    //     match self.kind {
    //         HeatingCoolingKind::IdealHeaterCooler => "IdealHeaterCooler",
    //         HeatingCoolingKind::ElectricHeating => "ElectricHeater",
    //     }
    // }

    pub fn can_heat(&self) -> bool {
        match self.kind {
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => true,
        }
    }

    pub fn can_cool(&self) -> bool {
        match self.kind {
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => false,
        }
    }

    
}
