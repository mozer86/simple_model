use std::rc::Rc;
use building_state_macro::BuildingObjectBehaviour;
use crate::simulation_state::SimulationState;
use crate::simulation_state_element::SimulationStateElement;
use crate::heating_cooling::HeaterCooler;

use crate::construction::Construction;
use crate::fenestration::Fenestration;
use crate::material::Material;
use crate::space::Space;
use crate::surface::Surface;
use crate::substance::Substance;

#[derive(Default, BuildingObjectBehaviour)]
pub struct Building {
    /// The name of the building
    pub name: String,
    
    // materiality
    pub substances: Vec<Rc<Substance>>,
    pub materials: Vec<Rc<Material>>,
    pub constructions: Vec<Rc<Construction>>,

    // geometry
    pub surfaces: Vec<Surface>,
    pub spaces: Vec<Space>,

    /// The windows and doors in the surface    
    pub fenestrations: Vec<Fenestration>,

    /// The Heating/Cooling devices in the space
    pub hvacs: Vec<HeaterCooler>,
    
}




impl Building {
        
    /// Maps the Physical [SimulationStateElements] into the building.
    /// 
    /// The rational here is that, after creating the Building object, the 
    /// construciton of Physics models will continue to add [SimulationStateElement]
    /// to the [SimulationState]. However, the process of creating these objects
    /// receives an immutable [Building] (i.e., `&Building`) and thus they cannot
    /// map them themselves. That is why we need this function.
    pub fn map_simulation_state(&mut self, state: &SimulationState)->Result<(),String>{
        // panic!("Fix!");
        let s = state.as_slice();
        let mut element_index = 0;
        while element_index < s.len() {
            match &s[element_index] {
                SimulationStateElement::Clothing(_)=>{
                    /* Added when calling Person::new()*/
                    unimplemented!()
                },
                SimulationStateElement::FenestrationOpenFraction(i, _)=>{
                    /* Added when calling Fenestration::new()*/
                    debug_assert!(self.fenestrations[*i].open_fraction_index().is_none());                    
                    self.fenestrations[*i].set_open_fraction_index(element_index);
                },
                SimulationStateElement::HeatingCoolingPowerConsumption(i, _)=>{
                    /* Added when calling HeaterCooler::new()*/
                    debug_assert!(self.hvacs[*i].heating_cooling_consumption_index().is_none());                    
                    self.hvacs[*i].set_heating_cooling_consumption_index(element_index);                    
                },
                SimulationStateElement::SpaceLightingPowerConsumption(_, _)=>{
                    /* Added when calling Luminaire::new()*/
                    unimplemented!()
                },
                SimulationStateElement::SpaceBrightness(space_index, _)=>{
                    debug_assert!(self.spaces[*space_index].brightness_index().is_none());                    
                    self.spaces[*space_index].set_brightness_index(element_index);                    
                },
                SimulationStateElement::SpaceDryBulbTemperature(space_index, _)=>{
                    debug_assert!(self.spaces[*space_index].dry_bulb_temperature_index().is_none());                    
                    self.spaces[*space_index].set_dry_bulb_temperature_index(element_index);                    
                },         
                SimulationStateElement::SpaceInfiltrationVolume(space_index,_)=>{
                    debug_assert!(self.spaces[*space_index].infiltration_volume_index().is_none());                    
                    self.spaces[*space_index].set_infiltration_volume_index(element_index);                    
                },   
                SimulationStateElement::SpaceVentilationVolume(space_index,_)=>{
                    debug_assert!(self.spaces[*space_index].ventilation_volume_index().is_none());                    
                    self.spaces[*space_index].set_ventilation_volume_index(element_index);                    
                },   
                SimulationStateElement::SpaceInfiltrationTemperature(space_index,_)=>{
                    debug_assert!(self.spaces[*space_index].infiltration_temperature_index().is_none());                    
                    self.spaces[*space_index].set_infiltration_temperature_index(element_index);                    
                },   
                SimulationStateElement::SpaceVentilationTemperature(space_index,_)=>{
                    debug_assert!(self.spaces[*space_index].ventilation_temperature_index().is_none());                    
                    self.spaces[*space_index].set_ventilation_temperature_index(element_index);                    
                },   
                SimulationStateElement::SpaceAirExchangeVolume(_origin,_target,_)=>{
                    unimplemented!()
                },    
                SimulationStateElement::SurfaceNodeTemperature(surface_index, _, _)=>{
                    // Check the first one
                    debug_assert!(self.surfaces[*surface_index].first_node_temperature_index().is_none());                                        
                    self.surfaces[*surface_index].set_first_node_temperature_index(element_index);                    

                    element_index += 1;
                    
                    // Fill the rest... loop until either the surface_index changes or the kind 
                    // of SimulationStateElement change
                    while let SimulationStateElement::SurfaceNodeTemperature(new_surface_index, _, _) = &s[element_index] {
                        if new_surface_index != surface_index {
                            break;
                        }
                        element_index += 1;
                        if element_index == s.len() {
                            debug_assert!(self.surfaces[*surface_index].last_node_temperature_index().is_none());                            
                            self.surfaces[*surface_index].set_last_node_temperature_index(element_index-1);                            
                            return Ok(())
                        }
                    }
                    
                    debug_assert!(self.surfaces[*surface_index].last_node_temperature_index().is_none());                    
                    self.surfaces[*surface_index].set_last_node_temperature_index(element_index-1);                    

                    // skip the increase in element_index that happens after the loop
                    continue; 
                },
                SimulationStateElement::FenestrationNodeTemperature(fen_index, _, _)=>{
                    // Check the first one
                    debug_assert!(self.fenestrations[*fen_index].first_node_temperature_index().is_none());                    
                    self.fenestrations[*fen_index].set_first_node_temperature_index(element_index);                    

                    element_index += 1;
                    
                    // Fill the rest... loop until either the surface_index changes or the kind 
                    // of SimulationStateElement change                    
                    while let SimulationStateElement::FenestrationNodeTemperature(new_surface_index, _, _) = &s[element_index] {                        
                        if new_surface_index != fen_index {
                            break;
                        }
                        element_index += 1;
                        if element_index == s.len() {
                            debug_assert!(self.fenestrations[*fen_index].last_node_temperature_index().is_none());                            
                            self.fenestrations[*fen_index].set_last_node_temperature_index(element_index-1);                            

                            return Ok(())
                        }
                    }
                    
                    debug_assert!(self.fenestrations[*fen_index].last_node_temperature_index().is_none());
                                        
                    self.fenestrations[*fen_index].set_last_node_temperature_index(element_index-1);                    

                    // skip the increase in element_index that happens after the loop
                    continue; 
                },
                SimulationStateElement::SpaceLoudness(space_index, _)=>{
                    debug_assert!(self.spaces[*space_index].loudness_index().is_none());                    
                    self.spaces[*space_index].set_loudness_index(element_index);                    
                },
            }// End of Match
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

    
    // #[test]
    // fn material() {
    //     let mut building = Building::new("The Building".to_string());

    //     // Add an empty material
    //     let mat_name = "The Material".to_string();
    //     let m0 = building.add_material(mat_name.clone());
    //     {
    //         let m = building.get_material(m0).unwrap();
    //         assert!(m.is_full().is_err());
    //     }

    //     let thickness = 3.21;
    //     building
    //         .set_material_properties(
    //             m0,
    //             MaterialProperties {
    //                 thickness: thickness,
    //             },
    //         )
    //         .unwrap();

    //     {
    //         let mat = building.get_material(m0).unwrap();
    //         assert!(mat.is_full().is_err());
    //         assert_eq!(mat.index(), 0);
    //     }

    //     // Add a couple of substances
    //     building.add_substance("Substance_0".to_string());
    //     let subs_name = "Substance_1".to_string();
    //     let s1 = building.add_substance(subs_name.clone());
    //     assert_eq!(1, s1);
    //     {
    //         let s = building.get_substance(s1).unwrap();
    //         assert_eq!(s.index(), s1);
    //     }

    //     // these should fail... index out of bounds
    //     assert!(building.set_material_substance(m0, 34).is_err());
    //     assert!(building.set_material_substance(131, s1).is_err());
    //     assert!(building.set_material_substance(131, 34).is_err());
    //     // this should work
    //     assert!(building.set_material_substance(m0, s1).is_ok());

    //     {
    //         let mat = building.get_material(m0).unwrap();
    //         assert_eq!(mat.index(), m0);
    //         assert_eq!(mat.thickness().unwrap(), thickness);
    //     }

    //     assert!(building.get_material(0).is_ok());
    //     assert!(building.get_material(1).is_err());
    //     assert!(building.get_material(2).is_err());
    // }

    // #[test]
    // fn construction() {
    //     let mut building = Building::new("The Building".to_string());

    //     // Add an empty material
    //     let mat_name = "The Material".to_string();
    //     let m0 = building.add_material(mat_name.clone());
    //     {
    //         let m = building.get_material(m0).unwrap();
    //         assert!(m.is_full().is_err());
    //     }

    //     let c_name = "The construction".to_string();
    //     let c0 = building.add_construction(c_name);

    //     assert!(building.add_material_to_construction(123, m0).is_err());
    //     assert!(building.add_material_to_construction(c0, 123).is_err());
    //     assert!(building.add_material_to_construction(123, 123).is_err());

    //     // This should work
    //     assert!(building.add_material_to_construction(c0, m0).is_ok());
    //     {
    //         let c = building.get_construction(c0).unwrap();
    //         assert_eq!(1, c.n_layers());
    //     }
    //     assert!(building.add_material_to_construction(c0, m0).is_ok());
    //     {
    //         let c = building.get_construction(c0).unwrap();
    //         assert_eq!(2, c.n_layers());
    //     }
    // }

    // #[test]
    // fn surface_space() {
    //     let mut building = Building::new("Test Building".to_string());

    //     let space_name = "Space 0".to_string();
    //     let space_index = building.add_space(space_name.clone());
    //     {
    //         let s = building.get_space(space_index).unwrap();
    //         assert_eq!(&space_name, s.name());
    //         assert_eq!(s.get_surfaces().len(), 0);
    //         assert_eq!(0, s.index());
    //         assert!(s.get_dry_bulb_temperature_state_index().is_none());
    //         assert!(s.is_full().is_err());
    //     }

    //     // Surface
    //     let s_name = "Surface 0".to_string();
    //     let s0 = building.add_surface(s_name.clone());
    //     {
    //         let s = building.get_surface(s0).unwrap();
    //         assert_eq!(&s_name, s.name());
    //         assert_eq!(0, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     building
    //         .set_surface_front_boundary(s0, Boundary::Space(space_index))
    //         .unwrap();
    //     building
    //         .set_surface_back_boundary(s0, Boundary::Ground)
    //         .unwrap();
    //     {
    //         let space = building.get_space(space_index).unwrap();
    //         let space_surfaces = space.get_surfaces();
    //         assert_eq!(space_surfaces.len(), 1);
    //         assert_eq!(space_surfaces[0], s0);

    //         let s = building.get_surface(s0).unwrap();
    //         if let Boundary::Space(i) = s.front_boundary() {
    //             assert_eq!(*i, space_index);
    //         } else {
    //             assert!(false);
    //         }

    //         if let Boundary::Ground = s.back_boundary() {
    //             assert!(true);
    //         } else {
    //             assert!(false);
    //         }

    //         assert_eq!(0, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     let s_name = "Surface 1".to_string();
    //     let _s1 = building.add_surface(s_name.clone());

    //     let s_name = "Surface 2".to_string();
    //     let s2 = building.add_surface(s_name.clone());
    //     {
    //         let s = building.get_surface(s2).unwrap();
    //         assert_eq!(&s_name, s.name());
    //         assert_eq!(2, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     building
    //         .set_surface_front_boundary(s2, Boundary::Ground)
    //         .unwrap();
    //     building
    //         .set_surface_back_boundary(s2, Boundary::Space(space_index))
    //         .unwrap();
    //     {
    //         let space = building.get_space(space_index).unwrap();
    //         let space_surfaces = space.get_surfaces();
    //         assert_eq!(space_surfaces.len(), 2);
    //         assert_eq!(space_surfaces[0], s0);
    //         assert_eq!(space_surfaces[1], s2);

    //         let s = building.get_surface(s2).unwrap();
    //         if let Boundary::Space(i) = s.back_boundary() {
    //             assert_eq!(*i, space_index);
    //         } else {
    //             assert!(false);
    //         }

    //         if let Boundary::Ground = s.front_boundary() {
    //             assert!(true);
    //         } else {
    //             assert!(false);
    //         }

    //         assert_eq!(2, s.index());
    //         assert!(s.is_full().is_err());
    //     }
    // }

    // use simulation_state::simulation_state_element::SimulationStateElement;

    // #[test]
    // fn fenestration_space() {
    //     let mut building = Building::new("Test Building".to_string());
    //     let mut state: SimulationState = SimulationState::new();

    //     let space_name_0 = "Space 0".to_string();
    //     let space_index_0 = building.add_space(space_name_0.clone());
    //     {
    //         let s = building.get_space(space_index_0).unwrap();
    //         assert_eq!(&space_name_0, s.name());
    //         assert_eq!(s.get_fenestrations().len(), 0);
    //         assert_eq!(0, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     let space_name_1 = "Space 1".to_string();
    //     let space_index_1 = building.add_space(space_name_1.clone());
    //     {
    //         let s = building.get_space(space_index_1).unwrap();
    //         assert_eq!(&space_name_1, s.name());
    //         assert_eq!(s.get_fenestrations().len(), 0);
    //         assert_eq!(1, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     // Fenestration
    //     let s_name = "Fen 0".to_string();
    //     let f0 = building.add_fenestration(
    //         &mut state,
    //         s_name.clone(),
    //         FenestrationPositions::FixedOpen,
    //         FenestrationType::Window,
    //     );
    //     {
    //         let f = building.get_fenestration(f0).unwrap();
    //         assert_eq!(&s_name, f.name());
    //         assert_eq!(0, f.index());
    //         assert!(f.is_full().is_err());

    //         assert!(f.operation_type() == FenestrationPositions::FixedOpen);

    //         assert_eq!(1, state.len());
    //         assert!(state[0] == SimulationStateElement::FenestrationOpenFraction(f0, 0.0));
    //     }

    //     building
    //         .set_fenestration_front_boundary(f0, Boundary::Space(space_index_0))
    //         .unwrap();
    //     building
    //         .set_fenestration_back_boundary(f0, Boundary::Space(space_index_1))
    //         .unwrap();
    //     {
    //         let space_0 = building.get_space(space_index_0).unwrap();
    //         let space_surfaces = space_0.get_fenestrations();
    //         assert_eq!(space_surfaces.len(), 1);
    //         assert_eq!(space_surfaces[0], f0);

    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         let space_surfaces = space_1.get_fenestrations();
    //         assert_eq!(space_surfaces.len(), 1);
    //         assert_eq!(space_surfaces[0], f0);

    //         let s = building.get_fenestration(f0).unwrap();
    //         if let Boundary::Space(i) = s.front_boundary() {
    //             assert_eq!(*i, space_index_0);
    //         } else {
    //             assert!(false);
    //         }

    //         if let Boundary::Space(i) = s.back_boundary() {
    //             assert_eq!(*i, space_index_1);
    //         } else {
    //             assert!(false);
    //         }

    //         assert_eq!(0, s.index());
    //         assert!(s.is_full().is_err());
    //     }

    //     let s_name = "Fen 1".to_string();
    //     let f1 = building.add_fenestration(
    //         &mut state,
    //         s_name.clone(),
    //         FenestrationPositions::Continuous,
    //         FenestrationType::Window,
    //     );
    //     assert_eq!(2, state.len());
    //     assert!(state[1] == SimulationStateElement::FenestrationOpenFraction(f1, 0.0));

    //     let s_name = "Fen 2".to_string();
    //     let f2 = building.add_fenestration(
    //         &mut state,
    //         s_name.clone(),
    //         FenestrationPositions::Continuous,
    //         FenestrationType::Window,
    //     );
    //     {
    //         let f = building.get_fenestration(f2).unwrap();
    //         assert_eq!(&s_name, f.name());
    //         assert_eq!(2, f.index());
    //         assert!(f.is_full().is_err());

    //         assert_eq!(3, state.len());
    //         assert!(state[2] == SimulationStateElement::FenestrationOpenFraction(f2, 0.0));
    //     }

    //     building
    //         .set_fenestration_front_boundary(f2, Boundary::Space(space_index_1))
    //         .unwrap();
    //     building
    //         .set_fenestration_back_boundary(f2, Boundary::Space(space_index_0))
    //         .unwrap();
    //     {
    //         let space = building.get_space(space_index_0).unwrap();
    //         let space_surfaces = space.get_fenestrations();
    //         assert_eq!(space_surfaces.len(), 2);
    //         assert_eq!(space_surfaces[0], f0);
    //         assert_eq!(space_surfaces[1], f2);

    //         let space = building.get_space(space_index_1).unwrap();
    //         let space_surfaces = space.get_fenestrations();
    //         assert_eq!(space_surfaces.len(), 2);
    //         assert_eq!(space_surfaces[0], f0);
    //         assert_eq!(space_surfaces[1], f2);

    //         let s = building.get_fenestration(f2).unwrap();
    //         if let Boundary::Space(i) = s.back_boundary() {
    //             assert_eq!(*i, space_index_0);
    //         } else {
    //             assert!(false);
    //         }

    //         if let Boundary::Space(i) = s.front_boundary() {
    //             assert_eq!(*i, space_index_1);
    //         } else {
    //             assert!(false);
    //         }

    //         assert_eq!(2, s.index());
    //         assert!(s.is_full().is_err());
    //     }
    // }

    // #[test]
    // fn test_heater_cooler() {
    //     let mut building = Building::new("Test Building".to_string());
    //     let mut state: SimulationState = SimulationState::new();

    //     let space_name_0 = "Space 0".to_string();
    //     let _ = building.add_space(space_name_0.clone());

    //     let space_name_1 = "Space 1".to_string();
    //     let space_index_1 = building.add_space(space_name_1.clone());

    //     assert_eq!(state.len(), 0);
    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         assert!(space_1.get_heating_cooling().is_none());
    //     }
    //     building
    //         .add_heating_cooling_to_space(
    //             &mut state,
    //             space_index_1,
    //             HeatingCoolingKind::IdealHeaterCooler,
    //         )
    //         .unwrap();
    //     assert_eq!(state.len(), 1);
    //     if let SimulationStateElement::SpaceHeatingCoolingPowerConsumption(space_index, power) =
    //         state[0]
    //     {
    //         assert_eq!(space_index_1, space_index);
    //         assert_eq!(0.0, power);
    //     }

    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         assert!(space_1.get_heating_cooling().is_some());
    //     }
    //     building
    //         .set_space_max_heating_power(space_index_1, 1500.)
    //         .unwrap();
    //     building
    //         .set_space_max_cooling_power(space_index_1, 2500.)
    //         .unwrap();

    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         let hc = space_1.get_heating_cooling().unwrap();
    //         assert_eq!(hc.max_cooling_power().unwrap(), 2500.);
    //         assert_eq!(hc.max_heating_power().unwrap(), 1500.);
    //     }
    // }

    // #[test]
    // fn test_luminaire() {
    //     let mut building = Building::new("Test Building".to_string());
    //     let mut state: SimulationState = SimulationState::new();

    //     let space_name_0 = "Space 0".to_string();
    //     let _ = building.add_space(space_name_0.clone());

    //     let space_name_1 = "Space 1".to_string();
    //     let space_index_1 = building.add_space(space_name_1.clone());

    //     assert_eq!(state.len(), 0);
    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         assert!(space_1.get_heating_cooling().is_none());
    //     }

    //     building
    //         .add_luminaire_to_space(&mut state, space_index_1)
    //         .unwrap();
    //     assert_eq!(state.len(), 1);
    //     if let SimulationStateElement::SpaceLightingPowerConsumption(space_index, light_power) =
    //         state[0]
    //     {
    //         assert_eq!(space_index_1, space_index);
    //         assert_eq!(light_power, 0.0);
    //     }

    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         assert!(space_1.get_luminaire().is_some());
    //         assert!(space_1.get_luminaire().unwrap().get_max_power().is_none());
    //     }

    //     building
    //         .set_space_max_lighting_power(space_index_1, 1500.)
    //         .unwrap();

    //     {
    //         let space_1 = building.get_space(space_index_1).unwrap();
    //         let lum = space_1.get_luminaire().unwrap();
    //         assert!(lum.get_max_power().is_some());
    //         assert_eq!(lum.get_max_power().unwrap(), 1500.);
    //     }
    // }
}
