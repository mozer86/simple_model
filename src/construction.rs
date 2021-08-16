use crate::object_trait::ObjectTrait;
use crate::building::Building;

/// An object representing a multilayer
/// Construction; that is to say, an array of
/// Materials
pub struct Construction {
    /// The name of the Construction object.
    /// Must be unique within the model
    name: String,

    /// The index of the Construction object within
    /// the constructions property in the Building object
    index: usize,

    /// The indices of the Material objects in the
    /// materials property of the Building object
    layers: Vec<usize>,

    // front finishing
    // back finishing  
}

impl ObjectTrait for Construction {
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        "construction".to_string()
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_full(&self) -> Result<(), String> {
        if !self.layers.is_empty() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }
}

impl Construction {
    /// Create a new empty Construction ...
    /// The index does not have any meaning if the Construction is
    /// self-contained; but it becomes meaningful when it is part of an
    /// Array. For instance, when inserting a new Construction to the     
    /// Building object, the latter chooses the appropriate index
    pub fn new(name: String, index: usize) -> Self {
        Construction {
            name,
            index,
            layers: Vec::new(),
        }
    }

    /// Borrows the Layers vector
    pub fn layers(&self) -> &Vec<usize> {
        &self.layers
    }

    /// Returns the number of layers in the object
    pub fn n_layers(&self) -> usize {
        self.layers.len()
    }

    /// Returns the number of the
    pub fn get_layer_index(&self, i: usize) -> Result<usize, String> {
        if self.layers.is_empty() {
            return self.error_using_empty();
        }

        match self.layers.get(i) {
            Some(v) => Ok(*v),
            None => {
                return Err(format!("Index out of bounds... trying to access layer {} of {} '{}', but it has only {} layers", i, self.class_name(), self.name, self.layers.len()));
            }
        }
    }

    /// adds another layer to the Construction.
    pub fn push_layer(&mut self, layer_index: usize) {
        self.layers.push(layer_index)
    }
}

impl Building{
    /* CONSTRUCTION */

    /// Creates a new construction
    pub fn add_construction(&mut self, name: String) -> usize {
        let i = self.constructions.len();
        self.constructions.push(Construction::new(name, i));
        i
    }

    /// Retrieves a construction
    pub fn get_construction(&self, index: usize) -> Result<&Construction, String> {
        if index >= self.constructions.len() {
            return self.error_out_of_bounds("Construction", index);
        }

        Ok(&self.constructions[index])
    }

    /// Pushes a new Material layer to a construction
    /// in the Building object
    pub fn add_material_to_construction(
        &mut self,
        construction_index: usize,
        material_index: usize,
    ) -> Result<(), String> {
        if material_index >= self.materials.len() {
            return self.error_out_of_bounds("Material", material_index);
        }

        if construction_index >= self.constructions.len() {
            return self.error_out_of_bounds("Construction", construction_index);
        }

        self.constructions[construction_index].push_layer(material_index);

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
        let name = "The construction".to_string();
        let index = 12312;
        let mut c = Construction::new(name.clone(), index);
        assert_eq!(&name, c.name());
        assert_eq!(index, c.index());
        assert_eq!(0, c.n_layers());
        assert!(c.is_full().is_err());

        let layer0 = 23;
        c.push_layer(layer0);
        assert_eq!(1, c.n_layers());
        assert_eq!(layer0, c.get_layer_index(0).unwrap());
        assert!(c.get_layer_index(1).is_err());

        let layer1 = 412;
        c.push_layer(layer1);
        assert_eq!(2, c.n_layers());
        assert_eq!(layer1, c.get_layer_index(1).unwrap());
        assert!(c.get_layer_index(1).is_ok());
        assert!(c.get_layer_index(2).is_err());
    }
}
