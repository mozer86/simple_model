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

use derive::{
    ObjectIO,     
    ObjectAPI
};

use geometry3d::{    
    Polygon3D,
    Loop3D
};


use std::rc::Rc;

use crate::model::SimpleModel;
use crate::boundary::Boundary;
use crate::construction::Construction;
use crate::simulation_state::{SimulationStateHeader, SimulationState};
use crate::simulation_state_element::{StateElementField, SimulationStateElement};

#[derive(Copy, Clone, Eq, PartialEq, ObjectIO)]
pub enum FenestrationPositions {
    FixedClosed,
    FixedOpen,
    Continuous,
    Binary,
}

#[derive(Copy, Clone, Eq, PartialEq, ObjectIO)]
pub enum FenestrationType {
    Window,
    Door,
}

/// A surface that can potentially be opened and closed.
/// It can be of any Construction and it does not need to be
/// a hole in another surface.
#[derive( ObjectIO, ObjectAPI, Clone)]
pub struct Fenestration {
    /// The name of the sub surface
    pub name: String,
    
    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// An array of Numbers representing the vertices of the 
    /// surface. The length of this array must be divisible by 3.
    pub vertices: Polygon3D,

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
    
    #[physical("front_temperature")]
    first_node_temperature: StateElementField,

    #[physical("back_temperature")]
    last_node_temperature: StateElementField,

    /// Index of the SimulationStateElement representing
    /// the fraction open in the SimulationState
    
    #[operational]
    open_fraction: StateElementField,

    
    #[physical]
    front_convection_coefficient: StateElementField,

    
    #[physical]
    back_convection_coefficient: StateElementField,

    
    #[physical]
    front_convective_heat_flow: StateElementField,

    
    #[physical]
    back_convective_heat_flow: StateElementField,

    
    #[physical]
    front_incident_solar_irradiance: StateElementField,

    
    #[physical]
    back_incident_solar_irradiance: StateElementField,

    
    #[physical]
    front_ir_irradiance: StateElementField, 

    
    #[physical]
    back_ir_irradiance: StateElementField,
}

impl Fenestration {
    /// Clones the outer [`Loop3D`] of the [`Fenestration`]
    pub fn clone_loop(&self) -> Loop3D {
        self.vertices.outer().clone()
    }

    /// Gets the area, based on the [`Polygon3D`] that represents
    /// this [`Fenestration`]
    pub fn area(&self) -> Float {
        self.vertices.area()
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

   
}










impl SimpleModel {

    /// Adds a [`Fenestration`] to the [`SimpleModel`]
    pub fn add_fenestration(&mut self, mut add : Fenestration, state: &mut SimulationStateHeader ) -> Rc<Fenestration>{
        // Check the index of this object
        let fen_index = self.fenestrations.len();
        add.set_index(fen_index);

        // Push the OpenFraction state, and map into the object
        let state_index = state.push( SimulationStateElement::FenestrationOpenFraction(fen_index), 0.);
        add.set_open_fraction_index(state_index);

        // Add to model, and return a reference
        let add = Rc::new(add);
        self.fenestrations.push(Rc::clone(&add));
        add
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {

    // use super::*;


}




