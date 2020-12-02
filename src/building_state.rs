use std::ops::{Index, IndexMut};
use std::mem;
use crate::building_state_element::BuildingStateElement;

/// The BuildingState is a Vector of BuildingStateElement objects.
/// It is intended to be a quick-to-clone structure.
/// 
/// To make it quicker to operate, the following conventions (never really checked)
/// are intended:
/// * Operational BuildingStateElement objects are all grouped at first. This is not a problem because they are added while creating the building.
/// * Elements cannot be repeated.
#[derive(Clone)]
pub struct BuildingState {
    n_operational: usize, 
    passed_operational: bool,
    elements: Vec<BuildingStateElement>
}

impl BuildingState {

    /// Creates a new empty Building State
    pub fn new()->Self{
        Self{
            n_operational: 0,
            passed_operational: false,
            elements: Vec::new()
        }
    }

    /// Pushes an BuildingStateElement into the elements
    /// vector.
    /// 
    /// It ensures that operational elements are there first, 
    /// and that physical are there afterwards.
    /// 
    /// Returns the index of the new element.
    pub fn push(&mut self, e : BuildingStateElement)-> usize {
        if e.is_operational(){
            if self.passed_operational {
                panic!("Trying to add an operational BuildingStateElement '{}' in between Physical ones", e.to_string());
            }
            self.n_operational += 1;
        } else {
            self.passed_operational = true;
        }

        let ret = self.elements.len();
        self.elements.push(e);
        return ret;
    }

    /// Returns the number of elements in the 
    /// State element
    pub fn len(&self)->usize{
        self.elements.len()
    }

    /// Returns the number of operational elements
    pub fn n_operational(&self)->usize{
        self.n_operational
    }

    /// Copies the Physical BuildingStateElements objects from origin to destination.
    /// It is expected that all the Operational BuildingStateElements are bundled at
    /// the beginning of the element.
    pub fn copy_physical_state_from(&mut self, origin: &BuildingState){
        
        debug_assert_eq!(origin.len(), self.len());
        
        for i in self.n_operational()..self.len(){
            // Check that these are of the same kind...
            debug_assert_eq!(mem::discriminant(&origin[i]), mem::discriminant(&self[i]));
            // Check that it is physical indeed
            debug_assert!(self[i].is_physical());
            self[i] = origin[i];            
        }
    }

    /// Copies the Operational BuildingStateElements objects from origin to destination.
    /// It is expected that all the Operational BuildingStateElements are bundled at
    /// the beginning of the element.
    pub fn copy_operational_state_from(&mut self, origin: &BuildingState){
        
        debug_assert_eq!(origin.len(), self.len());
        
        for i in 0..self.n_operational(){
            // Check that these are of the same kind...
            debug_assert_eq!(mem::discriminant(&origin[i]), mem::discriminant(&self[i]));
            // Check that it is physical indeed
            debug_assert!(self[i].is_operational());
            self[i] = origin[i];            
        }
    }
}

impl Index<usize> for BuildingState {
    type Output = BuildingStateElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl IndexMut<usize> for BuildingState {
    
    fn index_mut(&mut self, index: usize) -> &mut BuildingStateElement {
        &mut self.elements[index]
    }
}



/*
pub fn find_in_state(state: &BuildingState, element: BuildingStateElement )->Option<usize>{
    for i in 0..state.len(){
        if state[i] == element{
            return Some(i)
        }
    }
    None
}
*/





/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing{
    use super::*;

    #[test]
    fn test_push(){
        let mut state = BuildingState::new();
        assert_eq!(0, state.len());

        // Add one operational
        assert_eq!(0,state.push(BuildingStateElement::SpaceLightingPowerConsumption(0,1.0)));
        assert_eq!(1, state.len());
        assert_eq!(1, state.n_operational());
        assert!(!state.passed_operational);

        assert_eq!(1,state.push(BuildingStateElement::SpaceLightingPowerConsumption(0,1.0)));
        assert_eq!(2, state.len());
        assert_eq!(2, state.n_operational());
        assert!(!state.passed_operational);

        // push a physical one
        assert_eq!(2, state.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.)));
        assert_eq!(3, state.len());
        assert_eq!(2, state.n_operational());
        assert!(state.passed_operational);
    }

    #[test]
    #[should_panic]
    fn push_panic(){
        let mut state = BuildingState::new();

        // Add one operational
        state.push(BuildingStateElement::SpaceLightingPowerConsumption(0,1.0));
        
        // push a physical one
        state.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.));
        
        // Add an operational... it should panic now
        state.push(BuildingStateElement::SpaceLightingPowerConsumption(0,1.0));        
    }

    #[test]
    fn test_copy_physical_state(){

        /* CREATE STATE 1 */
        let mut state1 = BuildingState::new();

        // Add one operational
        state1.push(BuildingStateElement::SpaceLightingPowerConsumption(1,1.0));

        // push physical ones
        state1.push(BuildingStateElement::SpaceDryBulbTemperature(1,1.));        
        state1.push(BuildingStateElement::SpaceDryBulbTemperature(1,1.));


        /* CREATE STATE 2 */
        let mut state2 = BuildingState::new();

        // Add one operational
        state2.push(BuildingStateElement::SpaceLightingPowerConsumption(2,2.0));

        // push physical ones
        state2.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.));        
        state2.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.));
        
        // Copy
        state2.copy_physical_state_from(&state1);

        // Check that the first one, operational, was not touch in either.
        
        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state1[0]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state2[0]{
            assert_eq!(index,2);
            assert_eq!(value, 2.0);
        }else{
            assert!(false);
        }

        // Check that State 2 contains the elements in State 1        
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state1[1]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state2[1]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }

        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state1[2]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state2[2]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }


    }

    #[test]
    fn test_copy_operational_state(){

        /* CREATE STATE 1 */
        let mut state1 = BuildingState::new();

        // Add one operational
        state1.push(BuildingStateElement::SpaceLightingPowerConsumption(1,1.0));
        state1.push(BuildingStateElement::SpaceLightingPowerConsumption(1,1.0));

        // push physical ones
        state1.push(BuildingStateElement::SpaceDryBulbTemperature(1,1.));        
        state1.push(BuildingStateElement::SpaceDryBulbTemperature(1,1.));


        /* CREATE STATE 2 */
        let mut state2 = BuildingState::new();

        // Add one operational
        state2.push(BuildingStateElement::SpaceLightingPowerConsumption(2,2.0));
        state2.push(BuildingStateElement::SpaceLightingPowerConsumption(2,2.0));

        // push physical ones
        state2.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.));        
        state2.push(BuildingStateElement::SpaceDryBulbTemperature(2,2.));
        
        // Copy
        state2.copy_operational_state_from(&state1);

        // Check that the two operational states were transferred
        
        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state1[0]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state2[0]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }

        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state1[1]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceLightingPowerConsumption(index,value) = state2[1]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }

        // Check that the physical states were untouched
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state1[2]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state2[2]{
            assert_eq!(index,2);
            assert_eq!(value, 2.0);
        }else{
            assert!(false);
        }

        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state1[3]{
            assert_eq!(index,1);
            assert_eq!(value, 1.0);
        }else{
            assert!(false);
        }
        if let BuildingStateElement::SpaceDryBulbTemperature(index,value) = state2[3]{
            assert_eq!(index,2);
            assert_eq!(value, 2.0);
        }else{
            assert!(false);
        }


    }

}
