use crate::model::SimpleModel;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;
use crate::space::Space;
use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};

use crate::scanner::{Scanner, TokenType};

use std::rc::Rc;

#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
/// A Luminaire
/// 
/// Please fill this doc
pub struct Luminaire {
    /// The name of the Luminaire
    name: String,

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

