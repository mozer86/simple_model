use crate::simulation_state::SimulationState;
use crate::simulation_state_element::SimulationStateElement;
use crate::building::Building;
use crate::object_trait::ObjectTrait;

pub struct Luminaire {
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
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        "Luminaire".to_string()
    }

    
}

impl Luminaire {
    pub fn new(state: &mut SimulationState, name: String, space_index: usize) -> Self {
        // Push this to state.
        let state_index = state.len();
        state.push(
            // off by default,
            SimulationStateElement::SpaceLightingPowerConsumption(space_index, 0.0),
        );

        Self {
            name,
            state_index,
            index: 0,
            max_power: None,
        }
    }

    pub fn state_index(&self) -> usize {
        self.state_index
    }

    pub fn set_max_power(&mut self, p: f64) {
        self.max_power = Some(p);
    }

    pub fn get_max_power(&self) -> Option<f64> {
        self.max_power
    }
}

impl Building{
    /* LUMINAIRE */
    pub fn add_luminaire_to_space(
        &mut self,
        state: &mut SimulationState,
        space_index: usize,
    ) -> Result<(), String> {
        if space_index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", space_index);
        }

        self.spaces[space_index].add_luminaire(Luminaire::new(
            state,
            format!("Space {} Luminaire", space_index), // name
            space_index,
        ))
    }

    pub fn set_space_max_lighting_power(
        &mut self,
        space_index: usize,
        power: f64,
    ) -> Result<(), String> {
        if space_index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", space_index);
        }

        self.spaces[space_index].set_luminaire_max_power(power)
    }
}