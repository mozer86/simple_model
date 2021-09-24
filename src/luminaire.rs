use crate::building::Building;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::{SimulationStateElement,StateElementField};
use crate::space::Space;
use building_state_macro::{BuildingObjectBehaviour, SimpleInputOutput};

use crate::scanner::{Scanner, TokenType};

use std::rc::Rc;

#[derive(BuildingObjectBehaviour, SimpleInputOutput)]
/// A Luminaire
/// 
/// Please fill this doc
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
    /// calculation is performed, this can be left empty.
    target_space: Option<Rc<Space>>,

    /// The index of the state of the luminaire
    /// in the State array
    #[state]
    power_consumption: StateElementField,
}

impl Building {
    pub fn add_luminaire(
        &mut self,
        mut luminaire: Luminaire,
        state: &mut SimulationState,
    ) -> Rc<Luminaire> {
        let lum_index = self.luminaires.len();

        state.push(SimulationStateElement::LuminairePowerConsumption(
            lum_index, 0.,
        ));

        luminaire.set_index(lum_index);
        self.luminaires.push(Rc::new(luminaire));
        Rc::clone(self.luminaires.last().unwrap())
    }
}


