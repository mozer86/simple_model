use crate::heating_cooling::HeaterCooler;
use crate::luminaire::Luminaire;
use crate::object_trait::ObjectTrait;
use crate::building::Building;
use crate::heating_cooling::HeatingCoolingKind;
use calendar::date::Date;
use schedule::schedule_trait::Schedule;
use simulation_state::simulation_state::SimulationState;

/// Represents a space within a building. This will
/// often be a room, but it might also be half a room
pub struct Space {
    /// The name of the space
    name: String,

    /// The index of the Space in its containing array
    index: usize,

    /// Volume of the space
    volume: Option<f64>,

    /// The indices of the surrounding Surfaces in the
    /// Building's Surfaces array
    surfaces: Vec<usize>,

    /// The indices of the surrounding Fenestration in the
    /// Building's Surfaces array
    fenestrations: Vec<usize>,

    /// The Heating/Cooling devices in the space
    heating_cooling: Option<HeaterCooler>,

    /// The luminaire in the space
    luminaire: Option<Luminaire>,

    /// The importance of this space over time
    importance : Option<Box<dyn Schedule<f64>>>,

    /* STATE */
    /// The index of the DryBulb temperature of the space
    /// in the SimulationState array
    ///
    /// This will be filled by the Thermal Simulation module
    dry_bulb_temperature_state_index: Option<usize>,
    brightness_state_index: Option<usize>,
    loudness_state_index: Option<usize>,
    infiltration_vol_state_index: Option<usize>,
    infiltration_temp_state_index: Option<usize>,
    ventilation_vol_state_index: Option<usize>,
    ventilation_temp_state_index: Option<usize>,
}

impl ObjectTrait for Space {
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        "space".to_string()
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_full(&self) -> Result<(), String> {
        if self.volume.is_some() && !self.surfaces.is_empty() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }
}

impl Space {
    /// Creates a new Space
    pub fn new(name: String, index: usize) -> Self {
        // Add the zone to the State
        Self {
            index,
            name,

            volume: None,
            surfaces: Vec::new(),
            fenestrations: Vec::new(),
            heating_cooling: None,
            luminaire: None,
            importance: None,

            /* STATE */
            dry_bulb_temperature_state_index: None,
            brightness_state_index: None,
            loudness_state_index: None,
            infiltration_vol_state_index: None,
            infiltration_temp_state_index: None,
            ventilation_vol_state_index: None,
            ventilation_temp_state_index: None,
        }
    }

    /// Returns the volume of the space
    pub fn volume(&self) -> Result<f64, String> {
        match self.volume {
            Some(v) => Ok(v),
            None => self.error_using_empty(),
        }
    }

    /// Sets the volume of the space
    pub fn set_volume(&mut self, v: f64) {
        self.volume = Some(v);
    }

    /// Adds a surface reference to the Space's
    /// surfaces array
    pub fn push_surface(&mut self, s_index: usize) {
        self.surfaces.push(s_index)
    }

    /// retrieves the surfaces
    pub fn get_surfaces(&self) -> &Vec<usize> {
        &self.surfaces
    }
    
    /// Adds a fenestration reference to the Space's
    /// surfaces array
    pub fn push_fenestration(&mut self, s_index: usize) {
        self.fenestrations.push(s_index)
    }

    /// retrieves the fenestrations
    pub fn get_fenestrations(&self) -> &Vec<usize> {
        &self.fenestrations
    }

    pub fn get_importance(&self, time: Date) -> f64 {
        match &self.importance{
            Some(s)=> s.get(time).unwrap(),
            None => panic!("Space '{}' has no importance schedule", self.name())
        }
    }

    pub fn set_importance(&mut self, importance: Box<dyn Schedule<f64>>) {
        if self.importance.is_some(){
            panic!("Trying to replace Importance schedule in space '{}'", self.name())
        }
        self.importance = Some(importance);
    }



    pub fn get_dry_bulb_temperature_state_index(&self) -> Option<usize> {
        self.dry_bulb_temperature_state_index
    }

    pub fn set_dry_bulb_temperature_state_index(&mut self, i: usize) {
        self.dry_bulb_temperature_state_index = Some(i)
    }

    
    pub fn get_brightness_state_index(&self) -> Option<usize> {
        self.brightness_state_index
    }

    pub fn set_brightness_state_index(&mut self, i: usize) {
        self.brightness_state_index = Some(i)
    }

    pub fn get_loudness_state_index(&self) -> Option<usize> {
        self.loudness_state_index
    }

    pub fn set_loudness_state_index(&mut self, i: usize) {
        self.loudness_state_index = Some(i)
    }

    /* ********** */
    /* LUMINAIRES */
    /* ********** */

    /// Adds a Luminaire to the Space. Returns an error if there
    /// was a Luminaire already there.
    pub fn add_luminaire(&mut self, luminaire: Luminaire) -> Result<(), String> {
        if self.luminaire.is_some() {
            return Err(format!(
                "Trying to replace {} of {} '{}'",
                luminaire.class_name(),
                self.class_name(),
                self.name
            ));
        }
        self.luminaire = Some(luminaire);
        Ok(())
    }

    /// Sets the power for the luminaires in the Space.
    /// Returns an error if there are no luminaires in the Space.
    pub fn set_luminaire_max_power(&mut self, power: f64) -> Result<(), String> {
        match &mut self.luminaire {
            Some(h) => {
                h.set_max_power(power);
                Ok(())
            }
            None => Err(format!(
                "There are no Luminaires in {} '{}'",
                self.class_name(),
                self.name()
            )),
        }
    }

    /// Retrieves the Luminaire
    pub fn get_luminaire(&self) -> Option<&Luminaire> {
        match &self.luminaire {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Retrieves the state index of the Luminaires in the Space,
    /// if any
    pub fn get_luminaires_state_index(&self) -> Option<usize> {
        self.luminaire.as_ref().map(|h| h.state_index())
        // match &self.luminaire {
        //     Some(h) => Some(h.state_index()),
        //     None => None,
        // }
    }

    /* *************** */
    /* HEATING/COOLING */
    /* *************** */

    /// Retrieves the HeatingCooling
    pub fn get_heating_cooling(&self) -> Option<&HeaterCooler> {
        match &self.heating_cooling {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Retrieves the state index of the heating/cooling system
    pub fn get_heating_cooling_state_index(&self) -> Option<usize> {
        self.heating_cooling.as_ref().map(|h| h.state_index())
        // match &self.heating_cooling {
        //     Some(h) => Some(h.state_index()),
        //     None => None,
        // }
    }

    /// Adds a Heating/Cooling of the space. Returns an error if there was
    /// a system already there.
    pub fn add_heating_cooling(&mut self, system: HeaterCooler) -> Result<(), String> {
        if self.heating_cooling.is_some() {
            return Err(format!(
                "Trying to replace {} of {} '{}'",
                system.class_name(),
                self.class_name(),
                self.name
            ));
        }
        self.heating_cooling = Some(system);
        Ok(())
    }

    /// Sets the maximum heating power.
    pub fn set_max_heating_power(&mut self, power: f64) -> Result<(), String> {
        match &mut self.heating_cooling {
            Some(v) => {
                v.set_max_heating_power(power);
                Ok(())
            }
            None => Err(format!(
                "{} '{}' has no Heating/Cooling system... cannot set maximum heating power",
                self.class_name(),
                self.name()
            )),
        }
    }

    /// Sets the maximum cooling power.
    pub fn set_max_cooling_power(&mut self, power: f64) -> Result<(), String> {
        match &mut self.heating_cooling {
            Some(v) => {
                v.set_max_cooling_power(power);
                Ok(())
            }
            None => Err(format!(
                "{} '{}' has no heating/cooling system... cannot set maximum cooling power",
                self.class_name(),
                self.name()
            )),
        }
    }
}

impl Building{
    /* SPACES */

    /// Creates a new construction
    pub fn add_space(&mut self, name: String) -> usize {
        let i = self.spaces.len();
        let space = Space::new(name, i);
        self.spaces.push(space);

        // State is added within the Thermal model

        i
    }

    /// Retrieves a pace
    pub fn get_space(&self, index: usize) -> Result<&Space, String> {
        if index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", index);
        }
        Ok(&self.spaces[index])
    }

    /// Sets a space volume
    pub fn set_space_volume(&mut self, index: usize, volume: f64) -> Result<(), String> {
        if index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", index);
        }
        self.spaces[index].set_volume(volume);
        Ok(())
    }

    /// Sets a space importance schedule
    pub fn set_space_importance(&mut self, index: usize, importance: Box<dyn Schedule<f64>>) -> Result<(), String> {
        if index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", index);
        }
        self.spaces[index].set_importance(importance);
        Ok(())
    }

    /* HEATER AND COOLER */
    pub fn add_heating_cooling_to_space(
        &mut self,
        state: &mut SimulationState,
        space_index: usize,
        kind: HeatingCoolingKind,
    ) -> Result<(), String> {
        if space_index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", space_index);
        }

        // State is modified when creating Heating Cooling

        self.spaces[space_index].add_heating_cooling(HeaterCooler::new(
            state,
            format!("Space {} Heater/Cooler", space_index), // name
            space_index,
            kind,
        ))
    }

    pub fn set_space_max_heating_power(
        &mut self,
        space_index: usize,
        power: f64,
    ) -> Result<(), String> {
        if space_index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", space_index);
        }

        self.spaces[space_index].set_max_heating_power(power)
    }

    pub fn set_space_max_cooling_power(
        &mut self,
        space_index: usize,
        power: f64,
    ) -> Result<(), String> {
        if space_index >= self.spaces.len() {
            return self.error_out_of_bounds("Space", space_index);
        }

        self.spaces[space_index].set_max_cooling_power(power)
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
        let given_index = 12;
        let space = Space::new("The Space".to_string(), given_index);

        assert_eq!(space.index(), given_index);

        assert!(space.get_dry_bulb_temperature_state_index().is_none());
    }
}
