use std::rc::Rc;
use geometry3d::loop3d::Loop3D;
use geometry3d::polygon3d::Polygon3D;
use building_state_macro::BuildingObjectBehaviour;

use crate::building::Building;
use crate::construction::Construction;
use crate::boundary::Boundary;
// use crate::simulation_state::SimulationState;
// use crate::simulation_state_element::SimulationStateElement;

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
#[derive(Clone, BuildingObjectBehaviour)]
pub struct Fenestration {
    /// The name of the sub surface
    pub name: String,    
    
    /// The Polygon3D that represents
    /// the dimensions and size of the Fenestration
    pub polygon: Polygon3D,

    /// The index of the Construction object in the
    /// constructions property of the Building object    
    pub construction: Rc<Construction>,

    /// The opportunity for operating the Fenestration
    pub operation_type: FenestrationPositions,

    /// It it a window or a door, or...?
    pub fenestration_type: FenestrationType,

    // The index of the Shading device attached to the Fenestration
    // in the shading property of the Building object
    //shading: Option<usize>,

    /// The position of the [`Fenestration`] in its containing 
    /// array
    index: Option<usize>,

    /// A reference to the Boundary in front of the Fenestration
    front_boundary: Option<Boundary>,

    /// A reference to the Boundary in back of the Fenestration
    back_boundary: Option<Boundary>,

    #[state]
    first_node_temperature: Option<usize>,

    #[state]
    last_node_temperature: Option<usize>,

    /// Index of the SimulationStateElement representing
    /// the fraction open in the SimulationState
    #[state]
    open_fraction: Option<usize>,
}


impl Fenestration {
    

    /// Clones the outer [`Loop3D`] of the [`Fenestration`]
    pub fn clone_loop(&self) -> Loop3D {
        self.polygon.outer().clone()        
    }

    /// Gets the area, based on the [`Polygon3D`] that represents
    /// this [`Fenestration`]
    pub fn area(&self) -> f64{
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
    //     new_open: f64,
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
    //             if new_open.abs() > f64::EPSILON && (new_open - 1.0).abs()>f64::EPSILON {
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

impl Building{
    /* FENESTRATION */

    /// Creates a new Fenestration object
    pub fn add_fenestration(&mut self, mut fenestration: Fenestration) -> &Fenestration {
        
        fenestration.index = Some(self.fenestrations.len());            
        self.fenestrations.push(fenestration);
        self.fenestrations.last().unwrap()
    }

    
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
