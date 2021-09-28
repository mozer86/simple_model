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
use crate::material::Material;
use std::rc::Rc;
use building_state_macro::{SimpleInputOutput, SimpleObjectBehaviour};
use crate::scanner::{Scanner, TokenType};


/// An object representing a multilayer
/// Construction; that is to say, an array of
/// Materials
#[derive(SimpleInputOutput, SimpleObjectBehaviour)]
pub struct Construction {
    /// The name of the Construction object.
    /// Must be unique within the model
    pub name: String,

    /// The indices of the Material objects in the
    /// materials property of the SimpleModel object
    pub layers: Vec<Rc<Material>>,
    
    // front finishing
    // back finishing
}

impl Construction {
    /// Calculates the R-value of the Construction (not including surface coefficients).
    pub fn r_value(&self) -> Result<Float, String> {

        let mut r = 0.0;

        for material in self.layers.iter() {                        
            let lambda = material.substance.thermal_conductivity()?;
            r += material.thickness / lambda;
        }

        Ok(r)
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::substance::Substance;

    #[test]
    fn test_construction_basic() {
        let c_name = "The construction".to_string();

        let mut c = Construction::new(c_name.clone());
        assert_eq!(0, c.layers.len());
        assert_eq!(c_name, c.name);

        // Create substance
        let sub_name = "the_sub".to_string();
        let sub = Rc::new(Substance::new(sub_name.clone()));

        // Create a Material
        let mat_1_name = "mat_1".to_string();
        let mat_1_thickness = 0.12312;
        let mat_1 = Rc::new(Material::new(
            mat_1_name.clone(),
            Rc::clone(&sub),
            mat_1_thickness,
        ));

        c.layers.push(mat_1);
        assert_eq!(1, c.layers.len());
        assert_eq!(mat_1_name, c.layers[0].name);
        assert_eq!(mat_1_thickness, c.layers[0].thickness);

        let mat_2_name = "mat_2".to_string();
        let mat_2_thickness = 1.12312;
        let mat_2 = Rc::new(Material::new(
            mat_2_name.clone(),
            Rc::clone(&sub),
            mat_2_thickness,
        ));

        c.layers.push(mat_2);
        assert_eq!(2, c.layers.len());
        assert_eq!(mat_2_name, c.layers[1].name);
        assert_eq!(mat_2_thickness, c.layers[1].thickness);
    }

    #[test]
    fn test_construction_from_bytes(){
        let bytes = b" {
            name : \"A Material\",            
            substance : Substance {          
                    name: \"le substancia\", // some doc?
                    thermal_conductivity : 1.2,
                    specific_heat_capacity : 2.2,    
                    density : 3.2                    
                },
            thickness: 0.1            
        }
        ";

        

        let mut building = SimpleModel::new("the building".to_string());
        let mat = Material::from_bytes(bytes, &mut building).unwrap();

        let mat = building.add_material(mat);

        let bytes = b" {
            name : \"The Construction\",            
            layers: [
                \"A Material\"
            ]            
        }
        ";

        let construction = Construction::from_bytes(bytes, &mut building).unwrap();
        assert!(Rc::ptr_eq(&mat, &construction.layers[0]));

    }
}
