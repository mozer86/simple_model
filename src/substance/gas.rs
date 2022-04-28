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


use derive::ObjectIO;

/// Represent a common gas, with known physical properties
#[derive(Clone, ObjectIO)]
pub enum StandardGas{
    /// Air gas
    Air,

    /// Argon gas
    Argon,

    /// Krypton gas
    Krypton,

    /// Xenon gas
    Xenon,

}


/// Represents a Normal; that is to say, a physical
/// materiality with physical properties. The name Normal
/// has been chosen instead of Material to respect EnergyPlus'
/// and other software's terminology (which does not include
/// Substace, but it does include Material, which is essentially
/// a Normal with a thickness).
#[derive(Clone, ObjectIO)]
pub struct Gas {
    /// The name of the Normal. Should be unique for each
    /// Material in the SimpleModel object    
    pub name: String,
    
    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// A predefined gas
    kind: Option<StandardGas>,

    
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::model::SimpleModel;

    

    
    #[test]
    fn test_gas_from_bytes() {
        let bytes = b" {
            name : \"some gas\",            
            kind: StandardGas::Air(),
        }";

        let mut building = SimpleModel::new("the building".to_string());

        let _sub = Gas::from_bytes(1, bytes, &mut building).unwrap();

        // assert_eq!(sub.name, "A substance".to_string());
        // assert!((1.2 - sub.thermal_conductivity.unwrap()).abs() < EPSILON);
        // assert!((2.2 - sub.specific_heat_capacity.unwrap()).abs() < EPSILON);
        // assert!((3.2 - sub.density.unwrap()).abs() < EPSILON);
    }
}
