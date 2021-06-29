use crate::object_trait::ObjectTrait;
use crate::building::Building;

/// The representation of a physical layer-Material.
/// That is to say, a layer of a certain thickness
/// made of a certain Substance
pub struct Material {
    /// The name of the material object
    name: String,

    /// The position of this object in the materials vector
    /// owned by the Building object
    index: usize,

    /// The index of the substance of which
    /// the material is made of in vector referenced
    /// by its substances property
    substance: Option<usize>,

    /// The physical properties of the Material
    properties: Option<MaterialProperties>,
}

pub struct MaterialProperties {
    /// The physical thickness of this material, in meters
    pub thickness: f64,
}

impl ObjectTrait for Material {
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        "Material".to_string()
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_full(&self) -> Result<(), String> {
        if self.properties.is_some() && self.substance.is_some() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }
}

impl Material {
    /// Creates an empty Material. The index is irrelevant
    /// if the Material is not within an array. The Index
    /// value is chosen by the Building object when creating a new
    /// Material.
    pub fn new(name: String, index: usize) -> Self {
        Self {
            name,
            index,
            properties: None,
            substance: None,
        }
    }

    /// Sets the substance to the Material
    pub fn set_substance(&mut self, i: usize) {
        self.substance = Some(i);
    }

    /// Sets the substance to the Material
    pub fn set_properties(&mut self, properties: MaterialProperties) {
        self.properties = Some(properties);
    }

    /// Returns the thicnkess of this Material
    pub fn thickness(&self) -> Result<f64, String> {
        match &self.properties {
            Some(p) => Ok(p.thickness),
            None => self.error_using_empty(),
        }
    }

    /// Retrieves the substance index
    pub fn get_substance_index(&self) -> Option<usize> {
        self.substance
    }
}

impl Building{
        /* MATERIAL */

    /// Adds a new empty Material to the model
    pub fn add_material(&mut self, name: String) -> usize {
        let i = self.materials.len();
        self.materials.push(Material::new(name, i));
        i
    }

    /// Retrieves a material from the Materials array
    /// in the Building
    pub fn get_material(&self, index: usize) -> Result<&Material, String> {
        if index >= self.materials.len() {
            return self.error_out_of_bounds("Material", index);
        }

        Ok(&self.materials[index])
    }

    /// Sets a material surface
    pub fn set_material_substance(
        &mut self,
        material_index: usize,
        substance_index: usize,
    ) -> Result<(), String> {
        if material_index >= self.materials.len() {
            return self.error_out_of_bounds("Material", material_index);
        }

        if substance_index >= self.substances.len() {
            return self.error_out_of_bounds("Substance", substance_index);
        }

        self.materials[material_index].set_substance(substance_index);
        Ok(())
    }

    /// Sets a material property
    pub fn set_material_properties(
        &mut self,
        material_index: usize,
        properties: MaterialProperties,
    ) -> Result<(), String> {
        if material_index >= self.materials.len() {
            return self.error_out_of_bounds("Material", material_index);
        }

        self.materials[material_index].set_properties(properties);
        Ok(())
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
        let index = 123;
        let name = "The material".to_string();

        let mut s = Material::new(name.clone(), index);
        assert_eq!(&name, s.name());
        assert_eq!(s.index(), index);
        assert!(s.is_full().is_err());
        assert!(s.get_substance_index().is_none());

        // Fill with properties
        let thickness = 123123.123;
        s.set_properties(MaterialProperties {
            thickness: thickness,
        });
        assert!(s.is_full().is_err()); // substance missing.
        let substance: usize = 5124;
        s.set_substance(substance);
        assert!(s.is_full().is_ok()); // now it works.

        assert_eq!(s.get_substance_index().unwrap(), substance);
        assert_eq!(s.thickness().unwrap(), thickness);
    }
}
