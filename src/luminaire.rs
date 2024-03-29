/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use crate::model::SimpleModel;
use crate::simulation_state::SimulationStateHeader;
use crate::simulation_state_element::{SimulationStateElement, StateElementField};
use crate::space::Space;
use crate::Float;
use derive::{ObjectAPI, ObjectIO};

use std::rc::Rc;

/// A Luminaire
#[derive(ObjectIO, ObjectAPI, Clone)]
pub struct Luminaire {
    /// The name of the Luminaire
    name: String,

    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// The maximum power consumption
    max_power: Option<Float>,

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
    #[operational]
    power_consumption: StateElementField,
}

impl SimpleModel {
    /// Adds a [`Luminaire`] to the [`SimpleModel`]
    pub fn add_luminaire(
        &mut self,
        mut add: Luminaire,
        state: &mut SimulationStateHeader,
    ) -> Rc<Luminaire> {
        // Check the index of this object
        let obj_index = self.fenestrations.len();
        add.set_index(obj_index);

        // Push the state, and map into the object
        let state_index = state.push(
            SimulationStateElement::LuminairePowerConsumption(obj_index),
            0.,
        );
        add.set_power_consumption_index(state_index);

        // Add to model, and return a reference
        let add = Rc::new(add);
        self.luminaires.push(Rc::clone(&add));
        add
    }
}
