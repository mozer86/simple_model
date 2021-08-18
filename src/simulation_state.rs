use crate::simulation_state_element::SimulationStateElement;
use std::ops::Index;


#[cfg(debug_assertions)]
use std::mem;

/// The SimulationState is a Vector of SimulationStateElement objects.
/// It is intended to be a quick-to-clone structure.
///
/// To make it quicker to operate, the following conventions 
/// are enforced:
/// * Personal elements go first, then Operational go second, and Physical go third. (This is checked when pushing elements to the state) 
/// * Elements cannot be repeated (this is not really checked.).
#[derive(Default,Clone)]
pub struct SimulationState {
    /// The number of operational StateElements
    /// in the State
    n_operational: usize,

    /// The number of individual StateElements
    /// in the State
    n_individual: usize,

    /// A flag indicating whether any physical
    /// element has been assigned. If it has, then
    /// no more individual elements are allowed. This
    /// allows maintaining (this is a convention, made in favour
    /// of performance) all the individual elements at the
    /// beginning, then all the operational ones, and then
    /// all the physical ones
    passed_operational: bool,

    /// A flag indicating whether a non-individual
    /// element has been assigned. If it has, then
    /// no more individual elements are allowed. This
    /// allows maintaining (this is a convention, made in favour
    /// of performance) all the individual elements at the
    /// beginning, then all the operational ones, and then
    /// all the physical ones
    passed_personal: bool,

    /// The elements in the SimulationState
    elements: Vec<SimulationStateElement>,
}

impl SimulationState {
    /// Creates a new empty Building State
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a slice with its elements
    pub fn as_slice(&self) -> &[SimulationStateElement] {
        &self.elements[..]
    }

    /// Panics with an error message
    pub fn panic_corrupt(&self, exp: SimulationStateElement, found: SimulationStateElement) {
        panic!(
            "SimulationState: Corrupt SimulationState. Expecting {} but found {}",
            exp.to_string(),
            found.to_string()
        )
    }

    /// Borrows the StateElements of the State
    pub fn elements(&self) -> &Vec<SimulationStateElement> {
        &self.elements
    }

    /// Pushes an SimulationStateElement into the elements
    /// vector.
    ///
    /// It ensures that operational elements are there first,
    /// and that physical are there afterwards.
    ///
    /// Returns the index of the new element.
    pub fn push(&mut self, e: SimulationStateElement) -> usize {
        if e.is_personal() {
            // If individual, check if we are passed that... if not, count
            // and add
            if self.passed_personal {
                panic!("Trying to add an individual SimulationStateElement '{}' after no more were allowed", e.to_string());
            }
            self.n_individual += 1;
        } else if e.is_operational() {
            // If operational, check if we are passed that... if not, count
            // and add... mark individuals as passed
            if self.passed_operational {
                panic!("Trying to add an operational SimulationStateElement '{}' after no more were allowed", e.to_string());
            }
            self.passed_personal = true;
            self.n_operational += 1;
        } else {
            // Else, mark both individual and operational as pased.
            // Just add them
            self.passed_operational = true;
            self.passed_personal = true;
        }

        let ret = self.elements.len();
        self.elements.push(e);
        return ret;
    }

    /// Returns the number of elements in the
    /// State element
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns the number of operational elements
    pub fn n_operational(&self) -> usize {
        self.n_operational
    }

    /// Returns the number of operational elements
    pub fn n_individual(&self) -> usize {
        self.n_individual
    }

    /// Copies the whole state into another state
    pub fn copy_from(&mut self, origin: &SimulationState) {
        debug_assert_eq!(origin.len(), self.len());

        self.elements.copy_from_slice(&origin.as_slice())
    }

    /// Copies the Physical SimulationStateElements objects from origin to destination.
    /// It is expected that all the Operational BuildingStateElements are bundled at
    /// the beginning of the element.
    pub fn copy_physical_state_from(&mut self, origin: &SimulationState) {
        debug_assert_eq!(origin.len(), self.len());

        let ini = self.n_individual + self.n_operational;
        let fin = self.len();

        #[cfg(debug_assertions)]
        for i in ini..fin {
            // Check that these are of the same kind...
            debug_assert_eq!(mem::discriminant(&origin[i]), mem::discriminant(&self[i]));
            // Check that it is physical indeed
            debug_assert!(self[i].is_physical());
        }

        let origin_slice = origin.as_slice();
        self.elements[ini..fin].copy_from_slice(&origin_slice[ini..fin]);
    }

    /// Copies the Operational SimulationStateElements objects from origin to destination.
    /// It is expected that all the Operational BuildingStateElements are bundled at
    /// the beginning of the element.    
    pub fn copy_operational_state_from(&mut self, origin: &SimulationState) {
        debug_assert_eq!(origin.len(), self.len());

        let ini = self.n_individual;
        let fin = self.n_individual + self.n_operational;

        #[cfg(debug_assertions)]
        for i in ini..fin {
            // Check that these are of the same kind...
            debug_assert_eq!(mem::discriminant(&origin[i]), mem::discriminant(&self[i]));
            // Check that it is physical indeed
            debug_assert!(self[i].is_operational());
        }

        let origin_slice = origin.as_slice();
        self.elements[ini..fin].copy_from_slice(&origin_slice[ini..fin]);
    }

    /// Copies the Individual SimulationStateElements objects from origin to destination.
    /// It is expected that all the Operational BuildingStateElements are bundled at
    /// the beginning of the element.
    pub fn copy_individual_state_from(&mut self, origin: &SimulationState) {
        debug_assert_eq!(origin.len(), self.len());
        let ini = 0;
        let fin = self.n_individual;

        #[cfg(debug_assertions)]
        for i in ini..fin {
            // Check that these are of the same kind...
            debug_assert_eq!(mem::discriminant(&origin[i]), mem::discriminant(&self[i]));
            // Check that it is physical indeed
            debug_assert!(self[i].is_personal());
        }
        let origin_slice = origin.as_slice();
        self.elements[ini..fin].copy_from_slice(&origin_slice[ini..fin]);
    }

    /// Prints the Names of the Elements in the SimulationState
    pub fn print_header(&self) {
        for e in &self.elements {
            print!("{},", e.to_string())
        }
        println!();
    }

    /// Prints the values of the SimulationStateElements
    pub fn print_values(&self) {
        for e in &self.elements {
            print!("{},", e.get_value())
        }
        println!();
    }

    /// Replaces the [SimulationStateElement] in index `i` in a [SimulationState] by a `new_element`. 
    /// Panics if the new and old elements are not of the same variant or if their content differs in something
    /// else than in their final value.
    pub fn update_value(&mut self, i: usize, new_element: SimulationStateElement){
                
        if let Err(errmsg) = self[i].differ_only_in_value(new_element){
            panic!("Corrupt SimulationState: {}", errmsg);
        }

        // Replace
        self.elements[i]=new_element;
    }
    
}

impl Index<usize> for SimulationState {
    type Output = SimulationStateElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}



/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_new(){
        let state = SimulationState::new();
        assert_eq!(state.n_individual, 0);
        assert_eq!(state.passed_personal, false);
        assert_eq!(state.n_operational, 0);
        assert_eq!(state.passed_operational, false);
        assert_eq!(state.elements.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut state = SimulationState::new();
        assert_eq!(0, state.len());

        // Add one operational
        assert_eq!(
            0,
            state.push(SimulationStateElement::SpaceLightingPowerConsumption(
                0, 1.0
            ))
        );
        assert_eq!(1, state.len());
        assert_eq!(1, state.n_operational());
        assert!(state.passed_personal);
        assert!(!state.passed_operational);

        assert_eq!(
            1,
            state.push(SimulationStateElement::SpaceLightingPowerConsumption(
                0, 1.0
            ))
        );
        assert_eq!(2, state.len());
        assert_eq!(2, state.n_operational());
        assert!(!state.passed_operational);

        // push a physical one
        assert_eq!(
            2,
            state.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.))
        );
        assert_eq!(3, state.len());
        assert_eq!(2, state.n_operational());
        assert!(state.passed_operational);
    }

    #[test]
    #[should_panic]
    fn push_panic() {
        let mut state = SimulationState::new();

        // Add one operational
        state.push(SimulationStateElement::SpaceLightingPowerConsumption(
            0, 1.0,
        ));

        // push a physical one
        state.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));

        // Add an operational... it should panic now
        state.push(SimulationStateElement::SpaceLightingPowerConsumption(
            0, 1.0,
        ));
    }

    #[test]
    fn test_copy_operational_state() {
        /* CREATE STATE 1 */
        let mut state1 = SimulationState::new();

        // Add individual ones
        state1.push(SimulationStateElement::Clothing(1.0));
        state1.push(SimulationStateElement::Clothing(1.0));

        // Add operational ones
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));

        // push physical ones
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));

        /* CREATE STATE 2 */
        let mut state2 = SimulationState::new();

        // Add individual ones
        state2.push(SimulationStateElement::Clothing(2.0));
        state2.push(SimulationStateElement::Clothing(2.0));

        // Add operational ones
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));

        // push physical ones
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));

        // Copy
        state2.copy_operational_state_from(&state1);

        // CHECK

        // Check that the individual states were untouched
        if let SimulationStateElement::Clothing(value) = state1[0] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[0] {
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::Clothing(value) = state1[1] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[1] {
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        // Check that the two operational states were transferred

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[2] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[2] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[3] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[3] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }

        // Check that the physical states were untouched
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[4] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[4] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[5] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[5] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_copy_individual_state() {
        /* CREATE STATE 1 */
        let mut state1 = SimulationState::new();

        // Add individual ones
        state1.push(SimulationStateElement::Clothing(1.0));
        state1.push(SimulationStateElement::Clothing(1.0));

        // Add operational ones
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));

        // push physical ones
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));

        /* CREATE STATE 2 */
        let mut state2 = SimulationState::new();

        // Add individual ones
        state2.push(SimulationStateElement::Clothing(2.0));
        state2.push(SimulationStateElement::Clothing(2.0));

        // Add operational ones
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));

        // push physical ones
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));

        // Copy
        state2.copy_individual_state_from(&state1);

        // CHECK

        // Check that the individual states were copied
        if let SimulationStateElement::Clothing(value) = state1[0] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[0] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::Clothing(value) = state1[1] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[1] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }

        // Check that the two operational states are untouched

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[2] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[2] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[3] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[3] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        // Check that the physical states were untouched
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[4] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[4] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[5] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[5] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_copy_physical_state() {
        /* CREATE STATE 1 */
        let mut state1 = SimulationState::new();

        // Add individual ones
        state1.push(SimulationStateElement::Clothing(1.0));
        state1.push(SimulationStateElement::Clothing(1.0));

        // Add operational ones
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));
        state1.push(SimulationStateElement::SpaceLightingPowerConsumption(
            1, 1.0,
        ));

        // push physical ones
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));
        state1.push(SimulationStateElement::SpaceDryBulbTemperature(1, 1.));

        /* CREATE STATE 2 */
        let mut state2 = SimulationState::new();

        // Add individual ones
        state2.push(SimulationStateElement::Clothing(2.0));
        state2.push(SimulationStateElement::Clothing(2.0));

        // Add operational ones
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));
        state2.push(SimulationStateElement::SpaceLightingPowerConsumption(
            2, 2.0,
        ));

        // push physical ones
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));
        state2.push(SimulationStateElement::SpaceDryBulbTemperature(2, 2.));

        // Copy
        state2.copy_physical_state_from(&state1);

        // CHECK

        // Check that the individual states were untouched
        if let SimulationStateElement::Clothing(value) = state1[0] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[0] {
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::Clothing(value) = state1[1] {
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::Clothing(value) = state2[1] {
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        // Check that the two operational states are untouched

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[2] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[2] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state1[3] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceLightingPowerConsumption(index, value) = state2[3] {
            assert_eq!(index, 2);
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        // Check that the physical states were transferred
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[4] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[4] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }

        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state1[5] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
        if let SimulationStateElement::SpaceDryBulbTemperature(index, value) = state2[5] {
            assert_eq!(index, 1);
            assert_eq!(value, 1.0);
        } else {
            assert!(false);
        }
    }
}
