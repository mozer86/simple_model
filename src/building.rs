
use crate::substance::Substance;
use crate::material::Material;
use crate::construction::Construction;
use crate::object_trait::ObjectTrait;

use crate::surface::Surface;
use crate::space::Space;
//use crate::boundary::Boundary;

pub struct Building {

    // materiality
    substances: Vec<Substance>,
    materials: Vec<Material>,
    constructions: Vec<Construction>,

    // geometry
    surfaces: Vec<Surface>,
    spaces: Vec<Space>,

}


impl Building {

    /// Creates an empty building
    pub fn new() -> Self {
        Self{
            substances: Vec::new(),
            materials: Vec::new(),
            constructions: Vec::new(),
            surfaces: Vec::new(),
            spaces: Vec::new(),
        }
    }

    /// Checks whether the objects in the building are all full
    fn is_full(&self)-> Result<(),String> {

        for s in self.substances.iter(){
            if !s.is_full(){
                return s.error_is_not_full()
            }            
        }

        for s in self.materials.iter(){
            if !s.is_full(){
                return s.error_is_not_full()
            }
        }

        for s in self.constructions.iter(){
            if !s.is_full(){
                return s.error_is_not_full()
            }
        }
        
        for s in self.surfaces.iter(){
            if !s.is_full(){
                return s.error_is_not_full()
            }
        }

        for s in self.spaces.iter(){
            if !s.is_full(){
                return s.error_is_not_full()
            }
        }
        // All good
        Ok(())
    }

    

}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing{
    use super::*;

    
}