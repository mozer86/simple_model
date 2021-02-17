//use crate::error::error;
use crate::object_trait::ObjectTrait;

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
pub struct Substance {
    /// The name of the Substance. Should be unique for each
    /// Material in the Building object    
    name: String,

    /// The position of the Substance in its container Vector
    index: usize,

    properties: Option<SubstanceProperties>,
}

pub struct SubstanceProperties {
    /// The thermal conductivity of the substance in W/m.K
    pub thermal_conductivity: f64,

    /// The specific heat capacity of the substance in J/kg.K
    pub specific_heat_capacity: f64,

    /// The density of the substance in kg/m3
    pub density: f64,
}

impl ObjectTrait for Substance {
    fn name(&self) -> &String {
        &self.name
    }

    fn is_full(&self) -> Result<(), String> {
        if self.properties.is_some() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }

    fn index(&self) -> usize {
        self.index
    }

    fn class_name(&self) -> String {
        "Substance".to_string()
    }
}

impl Substance {
    /// Creates a new empty Surface with a certain index.ObjectTrait
    /// The index does not have any meaning if the Substance is
    /// self-contained; but it becomes meaningful when it is part of an
    /// Array. For instance, when inserting a new Substance to the     
    /// Building object, the latter chooses the appropriate index
    pub fn new(name: String, index: usize) -> Self {
        Self {
            name: name,
            index: index,
            properties: None,
        }
    }

    /// Fills the Substance with physical properties
    pub fn set_properties(&mut self, properties: SubstanceProperties) {
        self.properties = Some(properties)
    }

    /// Calculates the thermal diffusivity of the
    /// Substance
    pub fn thermal_diffusivity(&self) -> Result<f64, String> {
        match &self.properties {
            Some(p) => Ok(p.thermal_conductivity / (p.density * p.specific_heat_capacity)),
            None => self.error_using_empty(),
        }
    }

    /// Returns the thermal conductivity of the substance in W/m.K
    pub fn thermal_conductivity(&self) -> Result<f64, String> {
        match &self.properties {
            Some(p) => Ok(p.thermal_conductivity),
            None => self.error_using_empty(),
        }
    }

    /// The specific heat capacity of the substance in J/kg.K
    pub fn specific_heat_capacity(&self) -> Result<f64, String> {
        match &self.properties {
            Some(p) => Ok(p.specific_heat_capacity),
            None => self.error_using_empty(),
        }
    }

    /// The density of the substance in kg/m3
    pub fn density(&self) -> Result<f64, String> {
        match &self.properties {
            Some(p) => Ok(p.density),
            None => self.error_using_empty(),
        }
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_basic() {
        let lambda = 1.23123;
        let rho = 1.2312312555;
        let c = 9.123128;

        let index = 123;
        let s_name = "The Substance".to_string();
        let mut s = Substance::new(s_name.clone(), index);
        assert_eq!(&s_name, s.name());
        assert_eq!(s.index(), index);
        assert!(s.is_full().is_err());

        // Fill with properties
        s.set_properties(SubstanceProperties {
            thermal_conductivity: lambda,
            specific_heat_capacity: c,
            density: rho,
        });

        assert!(s.is_full().is_ok());

        assert_eq!(s.thermal_diffusivity().unwrap(), lambda / rho / c);
        assert_eq!(s.density().unwrap(), rho);
        assert_eq!(s.specific_heat_capacity().unwrap(), c);
    }
}
