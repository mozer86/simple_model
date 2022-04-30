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

use crate::model::SimpleModel;
use crate::substance::Substance;
use derive::ObjectIO;
use std::rc::Rc;

/// The representation of a physical layer-Material.
/// That is to say, a layer of a certain thickness
/// made of a certain Substance
#[derive(ObjectIO)]
pub struct Material {
    /// The name of the material object
    pub name: String,

    /// The position of this object in its contaner Vector
    index: Option<usize>,

    /// The [`Substance`] of which this
    /// [`Material`] is made of    
    pub substance: Substance,

    /// The thickness of the [`Material`]
    pub thickness: Float,
}

impl SimpleModel {
    /// Adds a [`Material`] to the [`SimpleModel`]
    pub fn add_material(&mut self, mut add: Material) -> Rc<Material> {
        add.set_index(self.materials.len());
        let add = Rc::new(add);
        self.materials.push(Rc::clone(&add));
        add
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::substance::Normal;

    #[cfg(feature = "float")]
    const EPSILON: f32 = std::f32::EPSILON;

    #[cfg(not(feature = "float"))]
    const EPSILON: f64 = std::f64::EPSILON;

    #[test]
    fn test_material_basic() {
        // We need a substance
        let sub_name = "sub_name".to_string();
        let substance = Normal::new(sub_name.clone()).wrap();

        // And a name
        let mat_name = "The material".to_string();

        // And a thickness
        let thickness = 123123.123;

        let s = Material::new(mat_name.clone(), substance.clone(), thickness);
        assert_eq!(mat_name, s.name);
        assert_eq!(sub_name, s.substance.name().clone());
        assert_eq!(thickness, s.thickness);
    }

    #[test]
    fn test_material_from_bytes() {
        /* BY NAME */

        let bytes = b" ::Normal {
            name : \"A substance\",            
            thermal_conductivity : 1.2,
            specific_heat_capacity : 2.2,    
            density : 3.2
        }
        ";

        let mut building = SimpleModel::new("the building".to_string());

        let sub = Substance::from_bytes(1, bytes, &mut building).unwrap();
        let sub = building.add_substance(sub);

        let bytes = b"{ 
            name : \"A Material\",            
            substance : \"A substance\",
            thickness: 0.1            
        }
        ";

        let mat = Material::from_bytes(1, bytes, &mut building).unwrap();

        assert_eq!(mat.name, "A Material".to_string());
        assert!((0.1 - mat.thickness).abs() < EPSILON);

        #[allow(irrefutable_let_patterns)]
        if let Substance::Normal(s1) = &mat.substance {
            if let Substance::Normal(s2) = &sub {
                assert!(Rc::ptr_eq(s1, s2));
            } else {
                panic!("pre aasd");
            }
        } else {
            panic!("asd")
        }

        /* SELF-CONTAINED DEFAULT */

        let bytes = b" {
            name : \"A Material\",            
            substance : Substance::Normal {          
                    name: \"le substancia\", // some doc?
                    thermal_conductivity : 1.2,
                    specific_heat_capacity : 2.2,    
                    density : 3.2
                },
            thickness: 0.1            
        }
        ";

        let mut building = SimpleModel::new("the building".to_string());
        let mat = Material::from_bytes(1, bytes, &mut building).unwrap();

        assert_eq!(mat.name, "A Material".to_string());
        assert!((0.1 - mat.thickness).abs() < EPSILON);

        #[allow(irrefutable_let_patterns)]
        if let Substance::Normal(sub) = mat.substance {
            assert_eq!(sub.name, "le substancia".to_string());
            assert!((1.2 - sub.thermal_conductivity().unwrap()).abs() < EPSILON);
            assert!((2.2 - sub.specific_heat_capacity().unwrap()).abs() < EPSILON);
            assert!((3.2 - sub.density().unwrap()).abs() < EPSILON);
        } else {
            panic!("Panic!!!!")
        }
    }
}
