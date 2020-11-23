use crate::object_trait::ObjectTrait;

use geometry3d::loop3d::Loop3D;
use geometry3d::polygon3d::Polygon3D;

use crate::construction::*;

pub enum FenestrationOperationType{
    FixedClosed,
    FixedOpen,
    Continuous,
    Binary,
}


/// A surface that can potentially be opened and closed
/// that should be a hole within a Surface 
/// Can be of any Construction
pub struct Fenestration {

    /// The name of the sub surface
    name: String,    

    /// The position of the SubSurface in its
    /// containing Array
    index: usize,

    /// The Polygon3D that represents
    /// the dimensions and size of the Surface
    polygon: Option<Polygon3D>,    

    /// The index of the Construction object in the 
    /// constructions property of the Building object    
    construction: Option<usize>,

    /// How much of the area is open
    open_fraction: f64,

    /// The index of the Shading device attached to the Fenestration
    /// in the shading property of the Building object
    shading: Option<usize>,

    /// The opportunity for operating the Fenestration
    operation_type: FenestrationOperationType
}

impl ObjectTrait for Fenestration {
    
    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->String{
        format!("Fenestration::{}",self.sub_class_name())
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->Result<(),String>{
        if self.polygon.is_some() && self.construction.is_some(){
            Ok(())
        }else{
            self.error_is_not_full()
        }
    }


}

impl Fenestration {

    /*
    pub fn new(name: String, l: Loop3D, construction: &Construction)-> Result<Self,String> {
        
        // Close loop if not closed
        if !l.is_closed(){
            return Err(format!("When creating SubSurface {} - Loop is not closed",name))
        }

        // Check if the polygon makes sense
        let p = match Polygon3D::new(l){
            Ok(v)=>v,
            Err(msg)=>{
                return Err(format!("When creating SubSurface {} - {}",name,msg))
            }
        };
        
        Ok(SubSurface{
            name: name,
            open_fraction: 0.0,
            construction:construction.index,
            polygon: p,
            shading: None::<usize>,
        })
    }
    */

    pub fn open_fraction(&self)->f64{
        self.open_fraction
    }

    pub fn get_construction_index(&self)->Result<usize,String>{
        match self.construction{
            Some(i)=>Ok(i),
            None => self.error_using_empty()
        }
    }

    /*
    pub fn clone_loop(&self)->Loop3D{
        self.polygon.clone_outer()
        
    }
    */

    pub fn area(&self)->Result<f64,String>{
        match &self.polygon {
            Some(p)=>Ok(p.area()),
            None => self.error_using_empty()
        }        
    }



    fn sub_class_name(&self)->&str{
        match self.operation_type {
            FenestrationOperationType::FixedClosed => "FixedClosed",
            FenestrationOperationType::FixedOpen => "FixedOpen",
            FenestrationOperationType::Continuous => "ContinuousOperation",
            FenestrationOperationType::Binary => "BinaryOperation",
        }
    }

    pub fn is_operable(&self) -> bool{
        match self.operation_type {
            FenestrationOperationType::FixedClosed => false,
            FenestrationOperationType::FixedOpen => false,
            FenestrationOperationType::Continuous => true,
            FenestrationOperationType::Binary => true,
        }
    }

    pub fn set_open_fraction(&mut self, new_open: f64) -> Result<(),String>{
                
        match self.operation_type {
            FenestrationOperationType::FixedClosed |
            FenestrationOperationType::FixedOpen => {
                Err(format!("Trying to operate a {}::{}: '{}'", self.class_name(),self.sub_class_name(), self.name))
            },
            FenestrationOperationType::Continuous => {
                self.open_fraction = new_open; 
                Ok(())
            },
            FenestrationOperationType::Binary => {
                if new_open != 0.0 && new_open != 1.0 {
                    return Err(format!("Trying leave '{}',  a {} {}, half-opened",self.name,self.sub_class_name(), self.class_name()));
                }else{
                    self.open_fraction = new_open;
                    return Ok(());
                }
            },
        }
    }

    


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
    fn test_fixed_closed(){

        // Crate a construction
        let s = Substance::new(
            "polyurethane".to_string(),
            0.0252, // W/m.K            
            2400., // J/kg.K
            17.5, // kg/m3... reverse engineered from paper            
        );

        let m = Material::new(s,0.1);        
        let c = Construction::new("Construction".to_string(),vec![Rc::clone(&m)]);

        // Test for closed
        // Geometry
        let mut the_loop = Loop3D::new();
        let l = 1. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();
                

        let mut fen = Fenestration::FixedClosed(
            SubSurface::new("FixedClosed".to_string(),the_loop,c).unwrap()
        );
          

        assert!(!fen.is_operable());
        assert!(fen.set_open_fraction(0.5).is_err());
        assert_eq!(fen.get_open_fraction(),0.0);
        
    }

    #[test]
    fn test_fixed_open(){

        // Crate a construction
        let s = Substance::new(
            "polyurethane".to_string(),
            0.0252, // W/m.K            
            2400., // J/kg.K
            17.5, // kg/m3... reverse engineered from paper            
        );

        let m = Material::new(s,0.1);        
        let c = Construction::new("Construction".to_string(),vec![Rc::clone(&m)]);

        // Test for closed
        // Geometry
        let mut the_loop = Loop3D::new();
        let l = 1. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();
            
        let mut fen = Fenestration::FixedOpen(
            SubSurface::new("FixedOpen".to_string(),the_loop,c).unwrap()
        );


        assert!(!fen.is_operable());
        assert!(fen.set_open_fraction(0.5).is_err());
        assert_eq!(fen.get_open_fraction(),1.0);
        
    }

    #[test]
    fn test_continuous(){

        // Crate a construction
        let s = Substance::new(
            "polyurethane".to_string(),
            0.0252, // W/m.K            
            2400., // J/kg.K
            17.5, // kg/m3... reverse engineered from paper            
        );

        let m = Material::new(s,0.1);        
        let c = Construction::new("Construction".to_string(),vec![Rc::clone(&m)]);

        // Test for closed
        // Geometry
        let mut the_loop = Loop3D::new();
        let l = 1. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();
        

        let mut fen = Fenestration::Continuous(
            SubSurface::new("Continuous".to_string(),the_loop,c).unwrap()
        );

        assert!(fen.is_operable());
        assert_eq!(fen.get_open_fraction(),0.0);
        
        assert!(fen.set_open_fraction(0.5).is_ok());
        assert_eq!(fen.get_open_fraction(),0.5);
        assert!(fen.set_open_fraction(0.1).is_ok());
        assert_eq!(fen.get_open_fraction(),0.1);
        assert!(fen.set_open_fraction(1.0).is_ok());
        assert_eq!(fen.get_open_fraction(),1.0);
    }

    #[test]
    fn test_binary(){

        // Crate a construction
        let s = Substance::new(
            "polyurethane".to_string(),
            0.0252, // W/m.K            
            2400., // J/kg.K
            17.5, // kg/m3... reverse engineered from paper            
        );

        let m = Material::new(s,0.1);        
        let c = Construction::new("Construction".to_string(),vec![Rc::clone(&m)]);

        // Test for closed
        // Geometry
        let mut the_loop = Loop3D::new();
        let l = 1. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();
        

        let mut fen = Fenestration::Binary(
            SubSurface::new("Binary".to_string(),the_loop,c).unwrap()
        );


        assert!(fen.is_operable());
        assert_eq!(fen.get_open_fraction(),0.0);

        assert!(fen.set_open_fraction(0.5).is_err());
        assert_eq!(fen.get_open_fraction(),0.0);

        assert!(fen.set_open_fraction(0.1).is_err());
        assert_eq!(fen.get_open_fraction(),0.0);

        assert!(fen.set_open_fraction(1.0).is_ok());
        assert_eq!(fen.get_open_fraction(),1.0);

        assert!(fen.set_open_fraction(0.0).is_ok());
        assert_eq!(fen.get_open_fraction(),0.0);
    }

*/
}