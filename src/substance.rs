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

// use std::cell::RefCell;
use crate::scanner::{Scanner,TokenType};

use crate::model::SimpleModel;
use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};

/// Represents a Substance; that is to say, a physical
/// materiality with physical properties. The name Substance
/// has been chosen instead of Material to respect EnergyPlus'
/// and other software's terminology (which does not include
/// Substace, but it does include Material, which is essentially
/// a Substance with a thickness).
#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
pub struct Substance {    
    /// The name of the Substance. Should be unique for each
    /// Material in the SimpleModel object    
    pub name: String,
    
    /// The thermal conductivity of the substance in W/m.K
    thermal_conductivity: Option<Float>,

    /// The specific heat capacity of the substance in J/kg.K
    specific_heat_capacity: Option<Float>,

    /// The density of the substance in kg/m3
    density: Option<Float>,
}

impl Substance {
    /// Calculates the thermal diffusivity of the
    /// Substance
    pub fn thermal_diffusivity(&self) -> Result<Float, String> {
        let thermal_conductivity = self.thermal_conductivity()?;
        let density = self.density()?;
        let specific_heat_capacity = self.specific_heat_capacity()?;
        Ok(thermal_conductivity / (density * specific_heat_capacity))
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
        assert_eq!(*s.density().unwrap(), rho);
        assert_eq!(*s.specific_heat_capacity().unwrap(), c);
        assert_eq!(*s.thermal_conductivity().unwrap(), lambda);
    }

    #[test]
    fn test_substance_from_bytes(){
        let bytes = b" {
            name : \"A substance\",            
            thermal_conductivity : 1.2,
            specific_heat_capacity : 2.2,    
            density : 3.2
        }";

        let mut building = SimpleModel::new("the building".to_string());

        let sub = Substance::from_bytes(bytes, &mut building).unwrap();

        assert_eq!(sub.name, "A substance".to_string());
        assert!((1.2 - sub.thermal_conductivity.unwrap()).abs()<EPSILON);
        assert!((2.2 - sub.specific_heat_capacity.unwrap()).abs()<EPSILON);
        assert!((3.2 - sub.density.unwrap()).abs()<EPSILON);

    }
}
