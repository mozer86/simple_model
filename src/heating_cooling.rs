use crate::building::Building;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::SimulationStateElement;
use building_state_macro::BuildingObjectBehaviour;

#[derive(Copy, Clone, Debug)]
pub enum HeatingCoolingKind {
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler,

    /// An electric heater, it can only
    /// heat.
    ElectricHeating,
}

impl HeatingCoolingKind {
    fn max_target_spaces(&self) -> Option<usize> {
        match self {
            Self::IdealHeaterCooler => None,
            Self::ElectricHeating => Some(1),
        }
    }
}

#[derive(Clone, BuildingObjectBehaviour)]
pub struct HeaterCooler {
    /// The name of the system
    pub name: String,

    /// The kind of heater utilized
    pub kind: HeatingCoolingKind,

    /// The `Space`s that this [`HeaterCooler`] heats and/or
    /// cools
    target_spaces: Vec<usize>,

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
    /// Checks whether this specific Heater/Cooler can heat
    pub fn can_heat(&self) -> bool {
        match self.kind {
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => true,
        }
    }

    /// Checks whether this specific Heater/Cooler can cool
    pub fn can_cool(&self) -> bool {
        match self.kind {
            HeatingCoolingKind::IdealHeaterCooler => true,
            HeatingCoolingKind::ElectricHeating => false,
        }
    }

    /// Borrows the spaces that this Heater/Cooler is serving
    pub fn target_spaces(&self) -> &Vec<usize> {
        &self.target_spaces
    }

    pub fn push_target_space(&mut self, i: usize) -> Result<(), String> {
        if let Some(max_targets) = self.kind.max_target_spaces() {
            // if there is a limit
            if self.target_spaces.len() >= max_targets {
                return Err(format!(
                    "Maximum number of target zones '{}' has been reached for HVAC {:?}",
                    max_targets, self.kind
                ));
            }
        }
        // there is no limit, or it has not been surpassed
        self.target_spaces.push(i);
        Ok(())
    }
}

impl Building {
    pub fn add_hvac(
        &mut self,
        mut hvac: HeaterCooler,
        state: &mut SimulationState,
    ) -> &HeaterCooler {
        let hvac_i = self.hvacs.len();
        // add element
        state.push(SimulationStateElement::HeatingCoolingPowerConsumption(
            hvac_i, 0.,
        ));

        // push
        hvac.set_index(hvac_i);
        self.hvacs.push(hvac);
        self.hvacs.last().unwrap()
    }
}
