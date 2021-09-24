use std::rc::Rc;
#[allow(dead_code)]
// use crate::fenestration::Fenestration;
// use crate::surface::Surface;

use building_state_macro::{BuildingObjectBehaviour, SimpleInputOutput};
use crate::building::Building;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::StateElementField;
use crate::scanner::{Scanner, TokenType};


/// Represents a space within a building. This will
/// often be a room, but it might also be half a room
#[derive(BuildingObjectBehaviour, SimpleInputOutput)]
pub struct Space {
    /// The name of the space
    pub name: String,

    /// Volume of the space
    pub volume: Option<f64>,

    /*
    /// The indices of the surrounding Surfaces in the
    /// Building's Surfaces array
    pub surfaces: Vec<Rc<RefCell<Surface>>>,

    /// The indices of the surrounding Fenestration in the
    /// Building's Surfaces array
    pub fenestrations: Vec<Rc<RefCell<Fenestration>>>,
    */

    /// The importance of this space over time
    // importance : Option<Box<dyn Schedule<f64>>>,

    /// The position of the [`Space`] in its containing
    /// array
    index: Option<usize>,

    #[state]
    dry_bulb_temperature: StateElementField,

    #[state]
    brightness: StateElementField,

    #[state]
    loudness: StateElementField,

    #[state]
    infiltration_volume: StateElementField,

    #[state]
    infiltration_temperature: StateElementField,

    #[state]
    ventilation_volume: StateElementField,

    #[state]
    ventilation_temperature: StateElementField,
}

impl Building {
    /* SPACES */

    /// Adds a [`Space`] to the [`Building`].
    ///
    /// The [`Space`] is put behind an `Rc`, and a clone
    /// of such `Rc` is returned
    pub fn add_space(&mut self, mut space: Space) -> Rc<Space> {
        space.set_index(self.spaces.len());

        self.spaces.push(Rc::new(space));
        Rc::clone(self.spaces.last().unwrap())
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

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
}
