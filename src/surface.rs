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

    // The windows and doors in the surface
    //fenestrations: Vec<Fenestration>,
    
}

impl ObjectTrait for Surface{
    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->&str{
        "Surface"
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->bool{
        self.construction.is_some() && self.polygon.is_some()
    }

}

/// A surface in the Building, separating two spaces, 
/// or a space and the exterior, or exterior and exterior
impl Surface {

    /*
    /// Creates a new surface that has no Space boundaries
    /// (i.e. it faces the exterior on both sides)
    /// # Arguments
    /// * name: The name of the surface
    /// * p: A Polygon3D which becomes the position and shape of the surface
    /// * c: The Construction which becomes the materiality of it
    pub fn new(name: String, p: Polygon3D, c: Rc<Construction>)-> Self {
        Self {
            polygon: p,
            construction: c,
            front_boundary: Boundary::None,
            back_boundary: Boundary::None,
            //fenestrations: Vec::new(),
            name: name,
        }
    }
    */

    /// Returns the area of the surface (calculated
    /// based on the Polygon3D that represents it)
    pub fn area(&self)->Result<f64,String>{
        match &self.polygon{
            Some(p)=>Ok(p.area()),
            None => self.error_using_empty()
        }
    }

    /// Returns the Construction object associated
    /// with the Surface
    pub fn get_construction_index(&self) -> Option<usize>{
        self.construction
    }

    /// Returns a reference to the front boundary
    pub fn front_boundary(&self) -> &Boundary {
        &self.front_boundary
    }

    /// Sets the front boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_front_boundary(&mut self, bound: Boundary) -> Result<(),String> {
        match self.front_boundary {
            Boundary::None => {
                self.front_boundary = bound;
                Ok(())
            },
            _ => {
                Err(format!("Trying to replace front boundary of {} '{}'",self.class_name(), self.name))
            }
        }        
    }

    /// Returns a reference to the back boundary
    pub fn back_boundary(&self) -> &Boundary {
        &self.back_boundary 
    }

    /// Sets the back boundary... does not let the Boundary know
    /// about this operation. The Building object handles that.
    pub fn set_back_boundary(&mut self, bound: Boundary) -> Result<(),String> {                
        match self.back_boundary{            
            // This should only work if there is no boundary already there            
            Boundary::None => {
                // Set the boundary
                self.back_boundary = bound;
                Ok(())
            },
            _ => {
                Err(format!("Trying to replace back boundary of {} '{}'",self.class_name(), self.name))
            }
        }        
    }

    /*
    /// Adds a fenestration to the surface.
    pub fn add_fenestration(&mut self, fenestration: Fenestration)->Result<(),String>{
        
        // Cut the hole
        match self.polygon.cut_hole(fenestration.clone_loop()){
            Err(msg)=>{
                return Err(format!("When trying to add fenestration {} to surface {} - {}",self.name, fenestration.name(), msg))
            },
            Ok(_) =>{}
        };
            
        
        // Add the fenestration
        self.fenestrations.push(fenestration);

        Ok(())
    }
    */


}


/***********/
/* TESTING */
/***********/



#[cfg(test)]
mod testing{
    /*
    use super::*;
    use crate::material::*;
    use crate::substance::*;
    
    use geometry3d::point3d::Point3D;
    use geometry3d::loop3d::Loop3D;

    #[test]
    fn test_add_fenestration(){

        // Crate a construction
        let s = Substance::new(
            "polyurethane".to_string(),
            0.0252, // W/m.K            
            2400., // J/kg.K
            17.5, // kg/m3... reverse engineered from paper            
        );

        let m = Material::new(s,0.1);        
        let c = Construction::new("Construction".to_string(),vec![Rc::clone(&m)]);
        
        // Geometry of hole
        let mut the_loop = Loop3D::new();
        let l = 1. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();

        // Geometry of surface
        let mut s_loop = Loop3D::new();
        let l = 1. as f64;
        s_loop.push( Point3D::new(-2.0*l, -2.0*l, 0.)).unwrap();
        s_loop.push( Point3D::new(2.0*l, -2.0*l, 0.)).unwrap();
        s_loop.push( Point3D::new(2.0*l, 2.0*l, 0.)).unwrap();
        s_loop.push( Point3D::new(-2.0*l, 2.0*l, 0.)).unwrap();
        s_loop.close().unwrap();
        
        
        
        // FixedClosed
        
        let p = Polygon3D::new(s_loop.clone()).unwrap();
        let mut surface = Surface::new("Surface".to_string(),p ,Rc::clone(&c));
        let fen = Fenestration::FixedClosed(
            SubSurface::new(
                "FixedClosed".to_string(),
                the_loop.clone(),
                Rc::clone(&c)
            ).unwrap()
        );

        assert_eq!(surface.area(),4.0*2.0*l*2.0*l);
        assert_eq!(surface.fenestrations.len(),0);

        assert!(surface.add_fenestration(fen).is_ok());

        assert_eq!(surface.area(),4.0*2.0*l*2.0*l - 4.0*l*l);

        assert_eq!(surface.fenestrations.len(),1);
        assert_eq!(surface.fenestrations[0].area(),4.0*l*l);

    }
    */
}




