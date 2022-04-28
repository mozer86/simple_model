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

pub mod normal;
pub mod gas;

pub use crate::substance::normal::Normal;
pub use crate::substance::gas::Gas;

use crate::model::SimpleModel;
use derive::{GroupAPI, GroupIO};
use std::rc::Rc;

/// A physical substance with physical—i.e., optical, thermal—properties.
/// 
/// Note that, contrary to EnergyPlus' `Materials`, `Substances` do not
/// contain information about the thickness, which in Simple is given when 
/// creating a `Material`. The idea is to enable multiple materials of different
/// thicknesses to reference the same material.
/// 
/// > Note: Glazing substances are `Normal` substances with `solar_transmitance` 
/// and `visible_transmittance`. However, contrary to all other properties, this property 
/// does depend on the thickness of the substance. So, in order
/// to build a coherent Glazing, you'll need to match this Substance
/// with an appropriate Material
#[derive(Clone, GroupAPI, GroupIO)]
pub enum Substance {

    /// A normal (i.e., solid, homogeneous) substance such as glass,
    /// timber or concrete.    
    Normal(Rc<Normal>),

    /// A gas
    Gas(Rc<Gas>),
}

impl SimpleModel {
    /// Adds a [`Substance`] to the [`SimpleModel`]
    pub fn add_substance(&mut self, mut add: Substance) -> Substance {
        // Check the index of this object
        let obj_index = self.substances.len();
        match &mut add {
            Substance::Normal(substance) => {
                let substance =
                    Rc::get_mut(substance).expect("Could not borrow Substance::Normal as mutable");
                substance.set_index(obj_index);
            },
            Substance::Gas(substance) => {
                let substance =
                    Rc::get_mut(substance).expect("Could not borrow Substance::Gas as mutable");
                substance.set_index(obj_index);
            }
        }

        // Add to model, and return a reference
        // let add = Rc::new(add);
        self.substances.push(add.clone());
        add
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_substance_normal_from_bytes() {
        let mut model = SimpleModel::new("the model".to_string());

        let bytes = b" ::Normal {
            name : \"Some substance\",            
            thermal_conductivity: 2.
        }
        ";

        let substance = Substance::from_bytes(1, bytes, &mut model);
        if let Ok(Substance::Normal(h)) = &substance {
            let found = h.thermal_conductivity().unwrap();
            let exp = 2.;

            assert!(
                (exp - found).abs() < 1e-5,
                "Expecting {}, but found {}",
                exp,
                found
            )
        } else {
            panic!("Definitely NOT an electric heater....!")
        }
    }

    #[test]
    fn test_substance_gas_from_bytes() {
        let mut model = SimpleModel::new("the model".to_string());

        let bytes = b" ::Gas {
            name : \"Some substance\",            
            kind: StandardGas::Air(),
        }
        ";

        let _substance = Substance::from_bytes(1, bytes, &mut model).unwrap();
        
    }
}
