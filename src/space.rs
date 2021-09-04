// use std::rc::Rc;
use building_state_macro::BuildingObjectBehaviour;
// use crate::surface::Surface;
// use crate::heating_cooling::HeaterCooler;
// use crate::luminaire::Luminaire;
use crate::building::Building;
// use crate::heating_cooling::HeatingCoolingKind;
// use calendar::date::Date;
// use schedule::schedule_trait::Schedule;
use crate::simulation_state::SimulationState;


/// Represents a space within a building. This will
/// often be a room, but it might also be half a room
#[derive(Clone, BuildingObjectBehaviour)]
pub struct Space {
    /// The name of the space
    pub name: String,
    
    /// Volume of the space
    pub volume: Option<f64>,

    /// The indices of the surrounding Surfaces in the
    /// Building's Surfaces array
    pub surfaces: Vec<usize>,

    /// The indices of the surrounding Fenestration in the
    /// Building's Surfaces array
    pub fenestrations: Vec<usize>,
    
    /// The luminaire in the space
    // luminaire: Option<Luminaire>,

    /// The importance of this space over time
    // importance : Option<Box<dyn Schedule<f64>>>,
    
    /// The position of the [`Space`] in its containing 
    /// array
    index: Option<usize>,
    
    #[state]
    dry_bulb_temperature: Option<usize>,

    #[state]
    brightness: Option<usize>,

    #[state]
    loudness: Option<usize>,

    #[state]
    infiltration_volume: Option<usize>,

    #[state]
    infiltration_temperature: Option<usize>,

    #[state]
    ventilation_volume: Option<usize>,

    #[state]
    ventilation_temperature: Option<usize>,

    
}




impl Building{
    /* SPACES */

    /// Adds a [`Space`] to the [`Building`].
    /// 
    /// The [`Space`] is put behind an `Rc`, and a clone
    /// of such `Rc` is returned
    pub fn add_space(&mut self, mut space: Space) -> &Space {
        space.set_index(self.spaces.len());
        
        self.spaces.push(space);
        self.spaces.last().unwrap()
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
        assert_eq!(space.volume().unwrap(), vol);
        

        let i = 91;
        assert!(space.dry_bulb_temperature.is_none());
        assert!(space.dry_bulb_temperature_index().is_none());
        space.set_dry_bulb_temperature_index(i);
        assert!(space.dry_bulb_temperature.is_some());
        assert_eq!(space.dry_bulb_temperature_index().unwrap(), i);

        let i = 191;
        assert!(space.brightness.is_none());
        assert!(space.brightness_index().is_none());
        space.set_brightness_index(i);
        assert!(space.brightness.is_some());
        assert_eq!(space.brightness_index().unwrap(), i);

        let i = 111;
        assert!(space.loudness.is_none());
        assert!(space.loudness_index().is_none());
        space.set_loudness_index(i);
        assert!(space.loudness.is_some());
        assert_eq!(space.loudness_index().unwrap(), i);
    }
}
