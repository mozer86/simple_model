use std::rc::Rc;


use crate::building::Building;
use building_state_macro::BuildingObjectBehaviour;

/// Represents a Substance; that is to say, a physical
/// materiality with physical properties. The name Substance
/// has been chosen instead of Material to respect EnergyPlus'
/// and other software's terminology (which does not include
/// Substace, but it does include Material, which is essentially
/// a Substance with a thickness).
///
/// All properties are public and no ::new() method is
/// defined because the number of properties of the Substance
/// object might grow quite a bit, and in the end it is simply
/// easier to write the struct down
#[derive(BuildingObjectBehaviour)]
pub struct Substance {
    /// The name of the Substance. Should be unique for each
    /// Material in the Building object    
    pub name: String,
        
    /// The position of the [`Substance`] in its containing 
    /// array
    index: Option<usize>,

    /// The thermal conductivity of the substance in W/m.K
    thermal_conductivity: Option<f64>,
    
    /// The specific heat capacity of the substance in J/kg.K
    specific_heat_capacity: Option<f64>,
    
    /// The density of the substance in kg/m3
    density: Option<f64>,
}


impl Substance {

    /// Calculates the thermal diffusivity of the
    /// Substance
    pub fn thermal_diffusivity(&self) -> Result<f64, String> {
        let thermal_conductivity = self.thermal_conductivity()?;
        let density = self.density()?;
        let specific_heat_capacity = self.specific_heat_capacity()?;
        Ok(thermal_conductivity / (density * specific_heat_capacity))
    }
}

impl Building {

    /// Adds a [`Substance`] to the [`Building`].
    /// 
    /// The [`Substance`] is put behind an `Rc`, and a clone
    /// of such `Rc` is returned
    pub fn add_substance(&mut self, substance: Substance) -> Rc<Substance> {
        let sub = Rc::new(substance);
        self.substances.push(Rc::clone(&sub));
        sub
    }

   
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;


    #[test]
    fn test_substance_basic() {
        
        let s_name = "The Substance".to_string();
        let mut s = Substance::new(s_name.clone());
        assert_eq!(s_name, s.name);
        assert!(s.thermal_conductivity().is_err());
        assert!(s.specific_heat_capacity().is_err());
        assert!(s.density().is_err());
        
        // Fill with properties
        let lambda = 1.23123;
        let rho = 1.2312312555;
        let c = 9.123128;
        s.set_thermal_conductivity(lambda)
            .set_specific_heat_capacity(c)
            .set_density(rho);
        
        
        assert_eq!(s.thermal_diffusivity().unwrap(), lambda / rho / c);
        assert_eq!(s.density().unwrap(), rho);
        assert_eq!(s.specific_heat_capacity().unwrap(), c);
        assert_eq!(s.thermal_conductivity().unwrap(), lambda);
    }
}
