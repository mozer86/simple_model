use geometry3d::loop3d::Loop3D;
use geometry3d::polygon3d::Polygon3D;

use crate::boundary::Boundary;
use crate::object_trait::ObjectTrait;
use simulation_state::simulation_state::SimulationState;
use simulation_state::simulation_state_element::SimulationStateElement;

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
pub struct Fenestration {
    /// The name of the sub surface
    name: String,

    /// The position of the Fenestration in its
    /// containing Array
    index: usize,

    /// The Polygon3D that represents
    /// the dimensions and size of the Fenestration
    polygon: Option<Polygon3D>,

    /// The index of the Construction object in the
    /// constructions property of the Building object    
    construction: Option<usize>,

    /// Index of the SimulationStateElement representing
    /// the fraction open in the SimulationState
    open_fraction_index: usize,

    // The index of the Shading device attached to the Fenestration
    // in the shading property of the Building object
    // shading: Option<usize>,
    /// The opportunity for operating the Fenestration
    operation_type: FenestrationPositions,

    /// It it a window or a door, or...?
    fenestration_type: FenestrationType,

    /// A reference to the Boundary in front of the Fenestration
    front_boundary: Boundary,

    /// A reference to the Boundary in back of the Fenestration
    back_boundary: Boundary,

    /* STATE */
    first_node_temperature_index: Option<usize>,
    last_node_temperature_index: Option<usize>,
}

impl ObjectTrait for Fenestration {
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        format!("Fenestration::{}", self.sub_class_name())
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_full(&self) -> Result<(), String> {
        if self.polygon.is_some() && self.construction.is_some() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }
}

impl Fenestration {
    /// Create a new empty Fenestration ...
    /// The index does not have any meaning if the Construction is
    /// self-contained; but it becomes meaningful when it is part of an
    /// Array. For instance, when inserting a new Construction to the     
    /// Building object, the latter chooses the appropriate index
    pub fn new(
        state: &mut SimulationState,
        name: String,
        index: usize,
        operation_type: FenestrationPositions,
        fenestration_type: FenestrationType,
    ) -> Self {
        // Push this to state.
        let open_index = state.len();
        state.push(
            // closed by default,
            SimulationStateElement::FenestrationOpenFraction(index, 0.0),
        );

        Self {
            name: name,
            index: index,
            operation_type: operation_type,
            fenestration_type: fenestration_type,
            open_fraction_index: open_index,
            polygon: None,
            construction: None,
            front_boundary: Boundary::None,
            back_boundary: Boundary::None,

            first_node_temperature_index: None,
            last_node_temperature_index: None,
        }
    }

    pub fn fenestration_type(&self) -> FenestrationType {
        self.fenestration_type
    }

    pub fn open_fraction(&self, state: &SimulationState) -> f64 {
        let i = self.open_fraction_index;

        if let SimulationStateElement::FenestrationOpenFraction(fen_index, open_fraction) = state[i]
        {
            if fen_index != self.index {
                panic!(
                    "Incorrect index allocated for OpenFraction of {} '{}'",
                    self.class_name(),
                    self.index
                );
            }

            // all Good here
            return open_fraction;
        } else {
            panic!("Incorrect StateElement kind allocated for OpenFraction of Fenestratoion of {} '{}'", self.class_name(), self.index);
        }
    }

    pub fn get_construction_index(&self) -> Result<usize, String> {
        match self.construction {
            Some(i) => Ok(i),
            None => self.error_using_empty(),
        }
    }

    pub fn clone_loop(&self) -> Result<Loop3D, String> {
        match &self.polygon {
            Some(p) => Ok(p.clone_outer()),
            None => self.error_using_empty(),
        }
    }

    pub fn area(&self) -> Result<f64, String> {
        match &self.polygon {
            Some(p) => Ok(p.area()),
            None => self.error_using_empty(),
        }
    }

    pub fn operation_type(&self) -> FenestrationPositions {
        self.operation_type
    }

    fn sub_class_name(&self) -> &str {
        match self.operation_type {
            FenestrationPositions::FixedClosed => "FixedClosed",
            FenestrationPositions::FixedOpen => "FixedOpen",
            FenestrationPositions::Continuous => "ContinuousOperation",
            FenestrationPositions::Binary => "BinaryOperation",
        }
    }

    pub fn is_operable(&self) -> bool {
        match self.operation_type {
            FenestrationPositions::FixedClosed => false,
            FenestrationPositions::FixedOpen => false,
            FenestrationPositions::Continuous => true,
            FenestrationPositions::Binary => true,
        }
    }

    pub fn set_polygon(&mut self, p: Polygon3D) {
        self.polygon = Some(p);
    }

    pub fn set_construction(&mut self, construction: usize) {
        self.construction = Some(construction)
    }

    pub fn set_open_fraction(
        &self,
        state: &mut SimulationState,
        new_open: f64,
    ) -> Result<(), String> {
        match self.operation_type {
            FenestrationPositions::FixedClosed | FenestrationPositions::FixedOpen => Err(format!(
                "Trying to operate a {}::{}: '{}'",
                self.class_name(),
                self.sub_class_name(),
                self.name
            )),
            FenestrationPositions::Continuous => {
                let i = self.open_fraction_index;

                if let SimulationStateElement::FenestrationOpenFraction(fen_index, _) = state[i] {
                    if fen_index != self.index {
                        panic!(
                            "Incorrect index allocated for OpenFraction of {} '{}'",
                            self.class_name(),
                            self.index
                        );
                    }

                    // all Good here
                    state[i] =
                        SimulationStateElement::FenestrationOpenFraction(fen_index, new_open);
                } else {
                    panic!("Incorrect StateElement kind allocated for OpenFraction of Fenestratoion of {} '{}'", self.class_name(), self.index);
                }

                Ok(())
            }
            FenestrationPositions::Binary => {
                if new_open != 0.0 && new_open != 1.0 {
                    return Err(format!(
                        "Trying leave '{}',  a {} {}, half-opened",
                        self.name,
                        self.sub_class_name(),
                        self.class_name()
                    ));
                } else {
                    let i = self.open_fraction_index;

                    if let SimulationStateElement::FenestrationOpenFraction(fen_index, _) = state[i]
                    {
                        if fen_index != self.index {
                            panic!(
                                "Incorrect index allocated for OpenFraction of {} '{}'",
                                self.class_name(),
                                self.index
                            );
                        }

                        // all Good here
                        state[i] =
                            SimulationStateElement::FenestrationOpenFraction(fen_index, new_open);
                    } else {
                        panic!("Incorrect StateElement kind allocated for OpenFraction of Fenestratoion of {} '{}'", self.class_name(), self.index);
                    }
                    return Ok(());
                }
            }
        }
    }

    /// Returns a reference to the front boundary
    pub fn front_boundary(&self) -> &Boundary {
        &self.front_boundary
    }

    /// Sets the front boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_front_boundary(&mut self, bound: Boundary) -> Result<(), String> {
        match self.front_boundary {
            Boundary::None => {
                if let Boundary::Ground = bound {
                    return Err(format!(
                        "Cannot set front boundary of {} '{}' to Ground",
                        self.class_name(),
                        self.name
                    ));
                } else {
                    self.front_boundary = bound;
                    return Ok(());
                }
            }
            _ => Err(format!(
                "Trying to replace front boundary of {} '{}'",
                self.class_name(),
                self.name
            )),
        }
    }

    /// Returns a reference to the back boundary
    pub fn back_boundary(&self) -> &Boundary {
        &self.back_boundary
    }

    /// Sets the back boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_back_boundary(&mut self, bound: Boundary) -> Result<(), String> {
        match self.back_boundary {
            // This should only work if there is no boundary already there
            Boundary::None => {
                if let Boundary::Ground = bound {
                    return Err(format!(
                        "Cannot set back boundary of {} '{}' to Ground",
                        self.class_name(),
                        self.name
                    ));
                } else {
                    // Set the boundary
                    self.back_boundary = bound;
                    return Ok(());
                }
            }
            _ => Err(format!(
                "Trying to replace back boundary of {} '{}'",
                self.class_name(),
                self.name
            )),
        }
    }

    pub fn set_first_node_temperature_index(&mut self, i: usize) {
        self.first_node_temperature_index = Some(i);
    }

    pub fn set_last_node_temperature_index(&mut self, i: usize) {
        self.last_node_temperature_index = Some(i);
    }

    pub fn get_first_node_temperature_index(&self) -> Option<usize> {
        self.first_node_temperature_index
    }

    pub fn get_last_node_temperature_index(&self) -> Option<usize> {
        self.last_node_temperature_index
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    #[should_panic]
    fn test_ground_boundary_front() {
        let mut state: SimulationState = SimulationState::new();
        let mut f = Fenestration::new(
            &mut state,
            format!("A"),
            12,
            FenestrationPositions::FixedOpen,
            FenestrationType::Window,
        );
        f.set_front_boundary(Boundary::Ground).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ground_boundary_back() {
        let mut state: SimulationState = SimulationState::new();
        let mut f = Fenestration::new(
            &mut state,
            format!("A"),
            12,
            FenestrationPositions::FixedOpen,
            FenestrationType::Window,
        );
        f.set_back_boundary(Boundary::Ground).unwrap();
    }
}
