use crate::object_trait::ObjectTrait;

use geometry3d::polygon3d::Polygon3D;

//use crate::fenestration::*;
use crate::boundary::*;

/// A fixed surface in the building (or surroundings). This can be of
/// any Construction, transparent or not.
pub struct Surface {
    /// The name of the surface
    name: String,

    /// The position of the Surface in the Building's Surface
    /// array
    index: usize,

    /// The Polygon3D that represents
    /// the dimensions and size of the Surface
    polygon: Option<Polygon3D>,

    /// The index of the construction in the Building's
    /// Construction array    
    construction: Option<usize>,

    /// A reference to the Boundary in front of the Surface
    front_boundary: Boundary,

    /// A reference to the Boundary in back of the Surface
    back_boundary: Boundary,

    /* STATE */
    first_node_temperature_index: Option<usize>,
    last_node_temperature_index: Option<usize>,
}

impl ObjectTrait for Surface {
    fn name(&self) -> &String {
        &self.name
    }

    fn class_name(&self) -> String {
        "Surface".to_string()
    }

    fn index(&self) -> usize {
        self.index
    }

    fn is_full(&self) -> Result<(), String> {
        if self.construction.is_some() && self.polygon.is_some() {
            Ok(())
        } else {
            self.error_is_not_full()
        }
    }
}

/// A surface in the Building, separating two spaces,
/// or a space and the exterior, or exterior and exterior
impl Surface {
    /// Creates a new empty Surface; that is, it has no Space boundaries
    /// (i.e. it faces the exterior on both sides), no construction
    /// and no polygon    
    pub fn new(name: String, index: usize) -> Self {
        Self {
            name,
            index,

            polygon: None,
            construction: None,
            front_boundary: Boundary::None,
            back_boundary: Boundary::None,

            first_node_temperature_index: None,
            last_node_temperature_index: None,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the area of the surface (calculated
    /// based on the Polygon3D that represents it)
    pub fn area(&self) -> Result<f64, String> {
        match &self.polygon {
            Some(p) => Ok(p.area()),
            None => self.error_using_empty(),
        }
    }

    /// Returns the Construction object associated
    /// with the Surface
    pub fn get_construction_index(&self) -> Option<usize> {
        self.construction
    }

    /// Sets the construction index
    pub fn set_construction(&mut self, construction: usize) {
        self.construction = Some(construction)
    }

    /// Returns a reference to the front boundary
    pub fn front_boundary(&self) -> &Boundary {
        &self.front_boundary
    }

    /// Sets the front boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_front_boundary(&mut self, bound: Boundary) -> Result<(), String> {
        match self.front_boundary {
            Boundary::None => {
                self.front_boundary = bound;
                Ok(())
            }
            _ => Err(format!(
                "Trying to replace front boundary of {} '{}'",
                self.class_name(),
                self.name
            )),
        }
    }

    /// Returns a reference to the back boundary
    pub fn back_boundary(&self) -> &Boundary {
        &self.back_boundary
    }

    /// Sets the back boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_back_boundary(&mut self, bound: Boundary) -> Result<(), String> {
        match self.back_boundary {
            // This should only work if there is no boundary already there
            Boundary::None => {
                // Set the boundary
                self.back_boundary = bound;
                Ok(())
            }
            _ => Err(format!(
                "Trying to replace back boundary of {} '{}'",
                self.class_name(),
                self.name
            )),
        }
    }

    pub fn set_polygon(&mut self, p: Polygon3D) {
        self.polygon = Some(p);
    }

    pub fn set_first_node_temperature_index(&mut self, i: usize) {
        self.first_node_temperature_index = Some(i);
    }

    pub fn set_last_node_temperature_index(&mut self, i: usize) {
        self.last_node_temperature_index = Some(i);
    }

    pub fn get_first_node_temperature_index(&self) -> Option<usize> {
        self.first_node_temperature_index
    }

    pub fn get_last_node_temperature_index(&mut self) -> Option<usize> {
        self.last_node_temperature_index
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
        // new
        let index = 12312;
        let name = "Some surface".to_string();
        let mut s0 = Surface::new(name.to_string(), index);
        assert!(s0.polygon.is_none());
        assert!(s0.construction.is_none());
        match s0.front_boundary() {
            Boundary::None => {}
            _ => assert!(false),
        };
        match s0.back_boundary() {
            Boundary::None => {}
            _ => assert!(false),
        }
        assert!(s0.is_full().is_err());
        assert!(s0.area().is_err());

        // set, get construction
        let construction_index = 9872;
        s0.set_construction(construction_index);
        if let Some(i) = s0.get_construction_index() {
            assert_eq!(i, construction_index);
        } else {
            assert!(false);
        }

        // set,get front boundary
        s0.set_front_boundary(Boundary::Ground).unwrap();
        match s0.front_boundary() {
            Boundary::Ground => {}
            _ => assert!(false),
        };
        s0.set_back_boundary(Boundary::Ground).unwrap();
        match s0.back_boundary() {
            Boundary::Ground => {}
            _ => assert!(false),
        };

        // polygon still missing
        assert!(s0.is_full().is_err());
    }
}
