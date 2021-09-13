use crate::building::Building;
use crate::substance::Substance;
use building_state_macro::BuildingObjectBehaviour;
use std::rc::Rc;
// use std::cell::RefCell;

/// The representation of a physical layer-Material.
/// That is to say, a layer of a certain thickness
/// made of a certain Substance
#[derive(BuildingObjectBehaviour)]
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
}
