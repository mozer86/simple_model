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

/// The kind of Floating point number used in the
/// library... the `"float"` feature means it becomes `f32`
/// and `f64` is used otherwise.
#[cfg(feature = "float")]
type Float = f32;

#[cfg(not(feature = "float"))]
type Float = f64;

pub mod rhai_api;
pub mod scanner;

mod simulation_state;
pub use simulation_state::{SimulationState, SimulationStateHeader};

mod simulation_state_element;
pub use simulation_state_element::SimulationStateElement;

mod model;
pub use model::SimpleModel;

mod building;
pub use building::{Building, ShelterClass};

mod construction;
pub use construction::Construction;

mod material;
pub use material::Material;

pub mod substance;
pub use substance::Substance;
// pub use substance;

mod boundary;
pub use boundary::Boundary;

mod fenestration;
pub use fenestration::{Fenestration, FenestrationPositions, FenestrationType};

mod luminaire;
pub use luminaire::Luminaire;

mod space;
pub use space::Space;

mod surface;
pub use surface::Surface;

mod infiltration;
pub use infiltration::Infiltration;

// Trait... better to leave this
pub mod hvac;
pub use hvac::HVAC;
