use crate::building::Building;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::SimulationStateElement;
use building_state_macro::BuildingObjectBehaviour;

#[derive(Clone, BuildingObjectBehaviour)]
pub struct Luminaire {
    /// The name of the Luminaire
    name: String,

    /// The position of the Luminaire in its
    /// containing Array (this is not used for now, as
    /// only one HeaterCooler is allowed per space)
    index: Option<usize>,

    /// The maximum power consumption
    max_power: Option<f64>,

    /// The space in which the space is located
    ///
    /// While this value is might not be relevant for
    /// e.g., lighting calculations, this is necessary for
    /// thermal simulations, in which the heat disipated by
    /// a luminaire will be disipated into the air of a thermal
    /// zone. So, if this is an exterior luminaire or if no thermal
    /// calculation is performed, this can be left to [`None`].
    target_space: Option<usize>,

    /// The index of the state of the luminaire
    /// in the State array
    #[state]
    power_consumption: Option<usize>,
}

impl Building {
    pub fn add_luminaire(
        &mut self,
        mut luminaire: Luminaire,
        state: &mut SimulationState,
    ) -> &Luminaire {
        let lum_index = self.luminaires.len();

        state.push(SimulationStateElement::LuminairePowerConsumption(
            lum_index, 0.,
        ));

        luminaire.set_index(lum_index);
        self.luminaires.push(luminaire);
        self.luminaires.last().unwrap()
    }
}
