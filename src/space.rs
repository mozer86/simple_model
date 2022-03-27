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
use std::rc::Rc;
use derive::{
    SimpleInputOutput, 
    SimpleRhaiAPI
};

use crate::model::SimpleModel;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;
use crate::infiltration::Infiltration;
use crate::building::Building;

/// Represents a space within a building. This will
/// often be a room, but it might also be half a room
#[derive(SimpleInputOutput, SimpleRhaiAPI, Clone)]
pub struct Space {
    /// The name of the space
    pub name: String,

    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// Volume of the space
    pub volume: Option<Float>,

    /// The infiltration in the space
    infiltration: Option<Infiltration>,

    /// The importance of this space over time
    // importance : Option<Box<dyn Schedule<Float>>>,
    building: Option<Rc<Building>>,
    
    #[physical]
    dry_bulb_temperature: StateElementField,

    #[physical]
    brightness: StateElementField,
    
    #[physical]
    loudness: StateElementField,
    
    #[physical]
    infiltration_volume: StateElementField,
    
    #[physical]
    infiltration_temperature: StateElementField,
    
    #[physical]
    ventilation_volume: StateElementField,
    
    #[physical]
    ventilation_temperature: StateElementField,
}



impl SimpleModel {

    /// Adds a [`Space`] to the [`SimpleModel`]
    pub fn add_space(&mut self, mut add : Space) -> Rc<Space>{
        add.set_index(self.spaces.len());
        let add = Rc::new(add);
        self.spaces.push(Rc::clone(&add));
        add
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;    

    #[cfg(feature = "float")]
    const EPSILON : f32 = std::f32::EPSILON;

    #[cfg(not(feature = "float"))]
    const EPSILON : f64 = std::f64::EPSILON;

    #[test]
    fn test_new() {
        let space_name = "the_space".to_string();

        let mut space = Space::new(space_name.clone());
        assert_eq!(space.name, space_name);
        assert!(space.volume().is_err());

        let vol = 987.12312;
        space.set_volume(vol);
        assert_eq!(*space.volume().unwrap(), vol);

        let i = 91;
        assert!(space.dry_bulb_temperature.borrow().is_none());
        assert!(space.dry_bulb_temperature_index().is_none());
        space.set_dry_bulb_temperature_index(i);
        assert!(space.dry_bulb_temperature.borrow().is_some());
        assert_eq!(space.dry_bulb_temperature_index().unwrap(), i);

        let i = 191;
        assert!(space.brightness.borrow().is_none());
        assert!(space.brightness_index().is_none());
        space.set_brightness_index(i);
        assert!(space.brightness.borrow().is_some());
        assert_eq!(space.brightness_index().unwrap(), i);

        let i = 111;
        assert!(space.loudness.borrow().is_none());
        assert!(space.loudness_index().is_none());
        space.set_loudness_index(i);
        assert!(space.loudness.borrow().is_some());
        assert_eq!(space.loudness_index().unwrap(), i);
    }

    #[test]
    fn test_space_from_bytes(){
        let bytes = b" {
            name : \"A Space\",            
            volume : 1.2,
            infiltration : Infiltration::Constant(2.2)
        }";

        let mut building = SimpleModel::new("the building".to_string());

        let space = Space::from_bytes(1, bytes, &mut building).unwrap();

        assert_eq!(space.name, "A Space".to_string());
        assert!((1.2 - space.volume.unwrap()).abs()<EPSILON);
        if let Some(Infiltration::Constant(v)) = space.infiltration{
            assert!((2.2 - v).abs()<EPSILON);
        }

    }
}
