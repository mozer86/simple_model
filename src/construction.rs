use crate::building::Building;
use crate::material::Material;
use std::rc::Rc;
use building_state_macro::{BuildingObjectBehaviour, SimpleInputOutput};
use crate::scanner::{Scanner, TokenType};


/// An object representing a multilayer
/// Construction; that is to say, an array of
/// Materials
#[derive(BuildingObjectBehaviour, SimpleInputOutput)]
pub struct Construction {
    /// The name of the Construction object.
    /// Must be unique within the model
    pub name: String,

    /// The indices of the Material objects in the
    /// materials property of the Building object
    pub layers: Vec<Rc<Material>>,

    index: Option<usize>,
    // front finishing
    // back finishing
}

impl Construction {
    /// Calculates the R-value of the Construction (not including surface coefficients).
    pub fn r_value(&self) -> Result<f64, String> {

        let mut r = 0.0;

        for material in self.layers.iter() {                        
            let lambda = material.substance.thermal_conductivity()?;
            r += material.thickness / lambda;
        }

        Ok(r)
    }
}

impl Building {
    /* CONSTRUCTION */

    /// Adds a [`Construction`] to the [`Building`].
    ///
    /// The [`Construction`] is put behind an `Rc`, and a clone
    /// of such `Rc` is returned
    pub fn add_construction(&mut self, mut construction: Construction) -> Rc<Construction> {
        construction.set_index(self.constructions.len());
        let ret = Rc::new(construction);
        self.constructions.push(Rc::clone(&ret));
        ret
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

        

        let mut building = Building::new("the building".to_string());
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
