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
use crate::Float;

use building_state_macro::{SimpleObjectBehaviour};
use geometry3d::loop3d::Loop3D;
use geometry3d::polygon3d::Polygon3D;
use std::rc::Rc;

use crate::boundary::Boundary;
use crate::construction::Construction;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FenestrationPositions {
    FixedClosed,
    FixedOpen,
    Continuous,
    Binary,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FenestrationType {
    Window,
    Door,
}

/// A surface that can potentially be opened and closed.
/// It can be of any Construction and it does not need to be
/// a hole in another surface.
#[derive(SimpleObjectBehaviour)]
pub struct Fenestration {
    /// The name of the sub surface
    pub name: String,
    
    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// The Polygon3D that represents
    /// the dimensions and size of the Fenestration
    pub polygon: Polygon3D,

    /// The index of the Construction object in the
    /// constructions property of the SimpleModel object    
    pub construction: Rc<Construction>,

    /// The opportunity for operating the Fenestration
    pub operation_type: FenestrationPositions,

    /// It it a window or a door, or...?
    pub fenestration_type: FenestrationType,

    // The index of the Shading device attached to the Fenestration
    // in the shading property of the SimpleModel object
    //shading: Option<usize>,
    
    /// A reference to the Boundary in front of the Fenestration
    front_boundary: Option<Boundary>,

    /// A reference to the Boundary in back of the Fenestration
    back_boundary: Option<Boundary>,

    #[state]
    first_node_temperature: StateElementField,

    #[state]
    last_node_temperature: StateElementField,

    /// Index of the SimulationStateElement representing
    /// the fraction open in the SimulationState
    #[state]
    open_fraction: StateElementField,
}

impl Fenestration {
    /// Clones the outer [`Loop3D`] of the [`Fenestration`]
    pub fn clone_loop(&self) -> Loop3D {
        self.polygon.outer().clone()
    }

    /// Gets the area, based on the [`Polygon3D`] that represents
    /// this [`Fenestration`]
    pub fn area(&self) -> Float {
        self.polygon.area()
    }

    // fn sub_class_name(&self) -> &str {
    //     match self.operation_type {
    //         FenestrationPositions::FixedClosed => "FixedClosed",
    //         FenestrationPositions::FixedOpen => "FixedOpen",
    //         FenestrationPositions::Continuous => "ContinuousOperation",
    //         FenestrationPositions::Binary => "BinaryOperation",
    //     }
    // }

    pub fn is_operable(&self) -> bool {
        match self.operation_type {
            FenestrationPositions::FixedClosed => false,
            FenestrationPositions::FixedOpen => false,
            FenestrationPositions::Continuous => true,
            FenestrationPositions::Binary => true,
        }
    }

    // pub fn set_open_fraction(
    //     &self,
    //     state: &mut SimulationState,
    //     new_open: Float,
    // ) -> Result<(), String> {
    //     match self.operation_type {
    //         FenestrationPositions::FixedClosed | FenestrationPositions::FixedOpen => Err(format!(
    //             "Trying to operate a {}::{}: '{}'",
    //             self.object_type(),
    //             self.sub_class_name(),
    //             self.name
    //         )),
    //         FenestrationPositions::Continuous => {
    //             let i = self.open_fraction_index()?;
    //             state.update_value(i, SimulationStateElement::FenestrationOpenFraction(self.index, new_open));
    //             Ok(())
    //         }
    //         FenestrationPositions::Binary => {
    //             if new_open.abs() > Float::EPSILON && (new_open - 1.0).abs()>Float::EPSILON {
    //                 return Err(format!(
    //                     "Trying leave '{}',  a {} {}, half-opened",
    //                     self.name,
    //                     self.sub_class_name(),
    //                     self.object_type()
    //                 ));
    //             } else {
    //                 let i = self.open_fraction_index()?;
    //                 state.update_value(i, SimulationStateElement::FenestrationOpenFraction(self.index, new_open));
    //                 Ok(())
    //             }
    //         }
    //     }
    // }
}


/***********/
/* TESTING */
/***********/

// #[cfg(test)]
// mod testing {

//     use super::*;

//     #[test]
//     #[should_panic]
//     fn test_ground_boundary_front() {
//         let mut state: SimulationState = SimulationState::new();
//         let mut f = Fenestration::new(
//             &mut state,
//             format!("A"),
//             12,
//             FenestrationPositions::FixedOpen,
//             FenestrationType::Window,
//         );
//         f.set_front_boundary(Boundary::Ground).unwrap();
//     }

//     #[test]
//     #[should_panic]
//     fn test_ground_boundary_back() {
//         let mut state: SimulationState = SimulationState::new();
//         let mut f = Fenestration::new(
//             &mut state,
//             format!("A"),
//             12,
//             FenestrationPositions::FixedOpen,
//             FenestrationType::Window,
//         );
//         f.set_back_boundary(Boundary::Ground).unwrap();
//     }
// }
