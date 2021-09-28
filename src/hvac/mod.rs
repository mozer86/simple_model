
mod ideal_heater_cooler;
mod electric_heater;

// use crate::space::Space;
use crate::model::SimpleModel;
use crate::hvac::ideal_heater_cooler::IdealHeaterCooler;
use crate::hvac::electric_heater::ElectricHeater;
use crate::scanner::{Scanner, TokenType};
use building_state_macro::GroupInputOutput;
use std::any::Any;

/// A collection of elements heating and cooling systems
#[derive(GroupInputOutput)]
pub enum HVACKind{
    /// An ideal heating/cooling device.
    /// Heats and Cools with an efficiency of
    /// 1, and nothing effects its COP or efficiency    
    IdealHeaterCooler,

    /// An electric heater, it can only
    /// heat.
    ElectricHeater
}





/// Shared functions for all objects in the [`HVAC`] group
pub trait HVAC {
    /// Gets the class of [`HVAC`], represented by the  
    /// variant within the [`HVACKind`] enum
    fn kind(&self)->HVACKind;

    /// Checks whether this specific [`HVAC`] can heat
    fn can_heat(&self)->bool;

    /// Checks whether this specific [`HVAC`] can cool
    fn can_cool(&self)->bool;

    /// Casts the [`HVAC`] object into an `Any`, which allows then 
    /// downcasting to the different kinds of [`HVAC`] 
    fn as_any(&self) -> &dyn Any;

    
    
}




/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::space::Space;
    use std::rc::Rc;

    #[test]
    fn test_hvac_from_bytes(){
        let mut building = SimpleModel::new("the building".to_string());
        let bytes = b"{
            name: \"the space\"
        }";
        let space = Space::from_bytes(bytes, &mut building).unwrap();
        assert_eq!(space.name, "the space".to_string());
        let space = building.add_space(space);

        let bytes = b" ::ElectricHeater {
            name : \"A heater\",            
            target_space: \"the space\"
        }
        ";

        let heater = HVACKind::from_bytes(bytes, &mut building).unwrap();
        match &heater.kind(){
            HVACKind::ElectricHeater =>{
                if let Some(h) = heater.as_any().downcast_ref::<ElectricHeater>() {                    
                    assert!(Rc::ptr_eq(&h.target_space, &space));
                } else {
                    panic!("Not an electric heater...?")
                }
            },
            HVACKind::IdealHeaterCooler=>{
                panic!("Definitely NOT an electric heater....!")
            }
        }
    }
}
