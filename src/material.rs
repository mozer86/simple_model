use crate::building::Building;
use crate::substance::Substance;
use std::rc::Rc;
use building_state_macro::{BuildingObjectBehaviour, SimpleInputOutput};
use crate::scanner::{Scanner, TokenType};



/// The representation of a physical layer-Material.
/// That is to say, a layer of a certain thickness
/// made of a certain Substance
#[derive(BuildingObjectBehaviour, SimpleInputOutput)]
pub struct Material {
    /// The name of the material object
    pub name: String,

    /// A reference to the [`Substance`] of which this
    /// [`Material`] is made of    
    pub substance: Rc<Substance>,

    /// The thickness of the [`Material`]
    pub thickness: f64,

    /// The position of the [`Material`] in its containing
    /// array
    index: Option<usize>,
}

impl Building {
    /// Adds a [`Material`] to the [`Building`].
    ///
    /// The [`Material`] is put behind an `Rc`, and a clone
    /// of such `Rc` is returned
    pub fn add_material(&mut self, material: Material) -> Rc<Material> {
        let ret = Rc::new(material);
        self.materials.push(Rc::clone(&ret));
        ret
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_material_basic() {
        // We need a substance
        let sub_name = "sub_name".to_string();
        let substance = Rc::new(Substance::new(sub_name.clone()));

        // And a name
        let mat_name = "The material".to_string();

        // And a thickness
        let thickness = 123123.123;

        let s = Material::new(mat_name.clone(), Rc::clone(&substance), thickness);
        assert_eq!(mat_name, s.name);
        assert_eq!(sub_name, s.substance.name);
        assert_eq!(thickness, s.thickness);
    }

    #[test]
    fn test_material_from_bytes(){

        /* BY NAME */

        let bytes = b" {
            name : \"A substance\",            
            thermal_conductivity : 1.2,
            specific_heat_capacity : 2.2,    
            density : 3.2
        }
        ";

        let mut building = Building::new("the building".to_string());

        let sub = Substance::from_bytes(bytes, &mut building).unwrap();
        let sub = building.add_substance(sub);

        let bytes = b"{ 
            name : \"A Material\",            
            substance : \"A substance\",
            thickness: 0.1            
        }
        ";

        let mat = Material::from_bytes(bytes, &mut building).unwrap();

        assert_eq!(mat.name, "A Material".to_string());
        assert!((0.1 - mat.thickness).abs()<std::f64::EPSILON);
        assert!(Rc::ptr_eq(&sub, &mat.substance));

        /* SELF-CONTAINED DEFAULT */
        
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

        assert_eq!(mat.name, "A Material".to_string());
        assert!((0.1  - mat.thickness).abs()<std::f64::EPSILON);
        let sub = &mat.substance;
        assert_eq!(sub.name, "le substancia".to_string());
        assert!((1.2 - sub.thermal_conductivity().unwrap()).abs()<std::f64::EPSILON);
        assert!((2.2 - sub.specific_heat_capacity().unwrap()).abs()<std::f64::EPSILON);
        assert!((3.2 - sub.density().unwrap()).abs()<std::f64::EPSILON);

    }
}
