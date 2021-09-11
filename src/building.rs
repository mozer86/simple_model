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
use std::cell::RefCell;

#[derive(Default, BuildingObjectBehaviour)]
pub struct Building {
    /// The name of the building
    pub name: String,

    // materiality
    pub substances: Vec<Rc<Substance>>,
    pub materials: Vec<Rc<Material>>,
    pub constructions: Vec<Rc<Construction>>,

    // geometry
    pub surfaces: Vec<Rc<RefCell<Surface>>>,
    pub spaces: Vec<Space>,

    /// The windows and doors in the surface    
    pub fenestrations: Vec<Fenestration>,

    /// The Heating/Cooling devices in the space
    pub hvacs: Vec<HeaterCooler>,

    /// Luminaires
    pub luminaires: Vec<Luminaire>,
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
        // panic!("Fix!");
        let s = state.as_slice();
        let mut element_index = 0;
        while element_index < s.len() {
            match &s[element_index] {
                SimulationStateElement::Clothing(_) => {
                    /* Added when calling Person::new()*/
                    unimplemented!()
                }
                SimulationStateElement::FenestrationOpenFraction(i, _) => {
                    debug_assert!(self.fenestrations[*i].open_fraction_index().is_none());
                    self.fenestrations[*i].set_open_fraction_index(element_index);
                }
                SimulationStateElement::HeatingCoolingPowerConsumption(i, _) => {
                    debug_assert!(self.hvacs[*i].heating_cooling_consumption_index().is_none());
                    self.hvacs[*i].set_heating_cooling_consumption_index(element_index);
                }
                SimulationStateElement::LuminairePowerConsumption(i, _) => {
                    debug_assert!(self.luminaires[*i].power_consumption_index().is_none());
                    self.luminaires[*i].set_power_consumption_index(element_index);
                }
                SimulationStateElement::SpaceBrightness(space_index, _) => {
                    debug_assert!(self.spaces[*space_index].brightness_index().is_none());
                    self.spaces[*space_index].set_brightness_index(element_index);
                }
                SimulationStateElement::SpaceDryBulbTemperature(space_index, _) => {
                    debug_assert!(self.spaces[*space_index]
                        .dry_bulb_temperature_index()
                        .is_none());
                    self.spaces[*space_index].set_dry_bulb_temperature_index(element_index);
                }
                SimulationStateElement::SpaceInfiltrationVolume(space_index, _) => {
                    debug_assert!(self.spaces[*space_index]
                        .infiltration_volume_index()
                        .is_none());
                    self.spaces[*space_index].set_infiltration_volume_index(element_index);
                }
                SimulationStateElement::SpaceVentilationVolume(space_index, _) => {
                    debug_assert!(self.spaces[*space_index]
                        .ventilation_volume_index()
                        .is_none());
                    self.spaces[*space_index].set_ventilation_volume_index(element_index);
                }
                SimulationStateElement::SpaceInfiltrationTemperature(space_index, _) => {
                    debug_assert!(self.spaces[*space_index]
                        .infiltration_temperature_index()
                        .is_none());
                    self.spaces[*space_index].set_infiltration_temperature_index(element_index);
                }
                SimulationStateElement::SpaceVentilationTemperature(space_index, _) => {
                    debug_assert!(self.spaces[*space_index]
                        .ventilation_temperature_index()
                        .is_none());
                    self.spaces[*space_index].set_ventilation_temperature_index(element_index);
                }
                SimulationStateElement::SpaceAirExchangeVolume(_origin, _target, _) => {
                    unimplemented!()
                }
                SimulationStateElement::SurfaceNodeTemperature(surface_index, _, _) => {
                    // Check the first one
                    let mut surface = self.surfaces[*surface_index].borrow_mut();
                    debug_assert!(surface
                        .first_node_temperature_index()
                        .is_none());
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
                            let mut surface = self.surfaces[*surface_index].borrow_mut();
                            debug_assert!(surface.last_node_temperature_index().is_none());
                            surface.set_last_node_temperature_index(element_index - 1);
                            return Ok(());
                        }
                    }

                    let mut surface = self.surfaces[*surface_index].borrow_mut();
                    debug_assert!(surface
                        .last_node_temperature_index()
                        .is_none());
                    surface.set_last_node_temperature_index(element_index - 1);

                    // skip the increase in element_index that happens after the loop
                    continue;
                }
                SimulationStateElement::FenestrationNodeTemperature(fen_index, _, _) => {
                    // Check the first one
                    debug_assert!(self.fenestrations[*fen_index]
                        .first_node_temperature_index()
                        .is_none());
                    self.fenestrations[*fen_index].set_first_node_temperature_index(element_index);

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
                            debug_assert!(self.fenestrations[*fen_index]
                                .last_node_temperature_index()
                                .is_none());
                            self.fenestrations[*fen_index]
                                .set_last_node_temperature_index(element_index - 1);

                            return Ok(());
                        }
                    }

                    debug_assert!(self.fenestrations[*fen_index]
                        .last_node_temperature_index()
                        .is_none());

                    self.fenestrations[*fen_index]
                        .set_last_node_temperature_index(element_index - 1);

                    // skip the increase in element_index that happens after the loop
                    continue;
                }
                SimulationStateElement::SpaceLoudness(space_index, _) => {
                    debug_assert!(self.spaces[*space_index].loudness_index().is_none());
                    self.spaces[*space_index].set_loudness_index(element_index);
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

    use geometry3d::polygon3d::Polygon3D;
    use geometry3d::loop3d::Loop3D;
    use geometry3d::point3d::Point3D;

    #[test]
    fn test_map_space(){
        let mut state = SimulationState::new();
        let mut building = Building::new("a building".to_string());

        let subs_name = "Substance 0".to_string();
        let substance = Substance::new(subs_name.clone());
        let substance = building.add_substance(substance);

        let mat_name = "Material 0".to_string();
        let material = Material::new(mat_name, substance, 0.1);
        let material = building.add_material(material);

        let con_name = "Construction 0".to_string();
        let mut construction = Construction::new(con_name);
        construction.layers.push(material);
        let construction = building.add_construction(construction);

        let mut the_loop = Loop3D::new();
        the_loop.push(Point3D::new(0.,0.,1.)).unwrap();
        the_loop.push(Point3D::new(0.,1.,1.)).unwrap();
        the_loop.push(Point3D::new(1.,0.,0.)).unwrap();
        the_loop.close().unwrap();
        let poly = Polygon3D::new(the_loop).unwrap();

        let surf_name = "The surface".to_string();
        let surf = Surface::new(surf_name, poly, construction);
        let surf = building.add_surface(surf);
        assert!(false);

    }
}
