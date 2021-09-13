use crate::heating_cooling::HeaterCooler;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::SimulationStateElement;
use building_state_macro::BuildingObjectBehaviour;
use std::rc::Rc;

use crate::construction::Construction;
use crate::fenestration::Fenestration;
use crate::luminaire::Luminaire;
use crate::material::Material;
use crate::space::Space;
use crate::substance::Substance;
use crate::surface::Surface;
// use std::cell::RefCell;

#[derive(Default, BuildingObjectBehaviour)]
pub struct Building {
    /// The name of the building
    pub name: String,

    // materiality
    pub substances: Vec<Rc<Substance>>,
    pub materials: Vec<Rc<Material>>,
    pub constructions: Vec<Rc<Construction>>,

    // geometry
    pub surfaces: Vec<Rc<Surface>>,
    pub spaces: Vec<Rc<Space>>,

    /// The windows and doors in the surface    
    pub fenestrations: Vec<Rc<Fenestration>>,

    /// The Heating/Cooling devices in the space
    pub hvacs: Vec<Rc<HeaterCooler>>,

    /// Luminaires
    pub luminaires: Vec<Rc<Luminaire>>,
}

impl Building {
    /// Maps the Physical [SimulationStateElement] into the building.
    ///
    /// The rational here is that, after creating the Building object, the
    /// construciton of Physics models will continue to add [SimulationStateElement]
    /// to the [SimulationState]. However, the process of creating these objects
    /// receives an immutable [Building] (i.e., `&Building`) and thus they cannot
    /// map them themselves. That is why we need this function.
    pub fn map_simulation_state(&mut self, state: &SimulationState) -> Result<(), String> {
        
        let s = state.as_slice();
        let mut element_index = 0;
        while element_index < s.len() {
            match &s[element_index] {
                SimulationStateElement::Clothing(_) => {
                    /* Added when calling Person::new()*/
                    unimplemented!()
                }
                SimulationStateElement::FenestrationOpenFraction(i, _) => {
                    let fenestration = &self.fenestrations[*i];
                    debug_assert!(fenestration.open_fraction_index().is_none());
                    fenestration.set_open_fraction_index(element_index);
                }
                SimulationStateElement::HeatingCoolingPowerConsumption(i, _) => {
                    let hvac = &self.hvacs[*i];
                    debug_assert!(hvac.heating_cooling_consumption_index().is_none());
                    hvac.set_heating_cooling_consumption_index(element_index);
                }
                SimulationStateElement::LuminairePowerConsumption(i, _) => {
                    let luminaire = &self.luminaires[*i];
                    debug_assert!(luminaire.power_consumption_index().is_none());
                    luminaire.set_power_consumption_index(element_index);
                }
                SimulationStateElement::SpaceBrightness(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.brightness_index().is_none());
                    space.set_brightness_index(element_index);
                }
                SimulationStateElement::SpaceDryBulbTemperature(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.dry_bulb_temperature_index().is_none());
                    space.set_dry_bulb_temperature_index(element_index);
                }
                SimulationStateElement::SpaceInfiltrationVolume(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.infiltration_volume_index().is_none());
                    space.set_infiltration_volume_index(element_index);
                }
                SimulationStateElement::SpaceVentilationVolume(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.ventilation_volume_index().is_none());
                    space.set_ventilation_volume_index(element_index);
                }
                SimulationStateElement::SpaceInfiltrationTemperature(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.infiltration_temperature_index().is_none());
                    space.set_infiltration_temperature_index(element_index);
                }
                SimulationStateElement::SpaceVentilationTemperature(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.ventilation_temperature_index().is_none());
                    space.set_ventilation_temperature_index(element_index);
                }
                SimulationStateElement::SpaceAirExchangeVolume(_origin, _target, _) => {
                    unimplemented!()
                }
                SimulationStateElement::SurfaceNodeTemperature(surface_index, _, _) => {
                    // Check the first one
                    let surface = &self.surfaces[*surface_index];
                    debug_assert!(surface.first_node_temperature_index().is_none());
                    surface.set_first_node_temperature_index(element_index);

                    element_index += 1;

                    // Fill the rest... loop until either the surface_index changes or the kind
                    // of SimulationStateElement change
                    while let SimulationStateElement::SurfaceNodeTemperature(
                        new_surface_index,
                        _,
                        _,
                    ) = &s[element_index]
                    {
                        if new_surface_index != surface_index {
                            break;
                        }
                        element_index += 1;
                        if element_index == s.len() {
                            let surface = &self.surfaces[*surface_index];
                            debug_assert!(surface.last_node_temperature_index().is_none());
                            surface.set_last_node_temperature_index(element_index - 1);
                            return Ok(());
                        }
                    }

                    let surface = &self.surfaces[*surface_index];
                    debug_assert!(surface
                        .last_node_temperature_index()
                        .is_none());
                    surface.set_last_node_temperature_index(element_index - 1);

                    // skip the increase in element_index that happens after the loop
                    continue;
                }
                SimulationStateElement::FenestrationNodeTemperature(fen_index, _, _) => {
                    let fenestration = &self.fenestrations[*fen_index];
                    // Check the first one
                    debug_assert!(fenestration.first_node_temperature_index().is_none());
                    fenestration.set_first_node_temperature_index(element_index);

                    element_index += 1;

                    // Fill the rest... loop until either the surface_index changes or the kind
                    // of SimulationStateElement change
                    while let SimulationStateElement::FenestrationNodeTemperature(
                        new_surface_index,
                        _,
                        _,
                    ) = &s[element_index]
                    {
                        if new_surface_index != fen_index {
                            break;
                        }
                        element_index += 1;
                        if element_index == s.len() {
                            let fenestration = &self.fenestrations[*fen_index];
                            debug_assert!(fenestration.last_node_temperature_index().is_none());
                            fenestration.set_last_node_temperature_index(element_index - 1);

                            return Ok(());
                        }
                    }

                    debug_assert!(fenestration.last_node_temperature_index().is_none());

                    fenestration.set_last_node_temperature_index(element_index - 1);

                    // skip the increase in element_index that happens after the loop
                    continue;
                }
                SimulationStateElement::SpaceLoudness(space_index, _) => {
                    let space = &self.spaces[*space_index];
                    debug_assert!(space.loudness_index().is_none());
                    space.set_loudness_index(element_index);
                }
            } // End of Match
            element_index += 1;
        }

        Ok(())
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    // use crate::substance::SubstanceProperties;
    // use crate::heating_cooling::HeatingCoolingKind;
    // use crate::boundary::Boundary;
    // use crate::fenestration::{FenestrationPositions, FenestrationType};

    #[test]
    fn building_substance() {
        let mut building = Building::new("Test Building".to_string());

        let subs_name = "Substance 0".to_string();
        let substance = Substance::new(subs_name.clone());

        let s0 = building.add_substance(substance);

        let s = &building.substances[0];
        assert_eq!(subs_name, s.name);
        assert_eq!(subs_name, s0.name);
    }

    
}
