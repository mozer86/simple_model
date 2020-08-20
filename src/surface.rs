use std::rc::Rc;
use geometry3d::polygon3d::Polygon3D;

use crate::construction::*;

pub struct Surface {    
    polygon: Polygon3D,    
    construction: Rc<Construction>,
}

impl Surface{

    pub fn new(p: Polygon3D, c: Rc<Construction>)->Rc<Surface>{
        Rc::new(Surface{
            polygon: p,
            construction: c,
        })
    }

    pub fn area(&self)->f64{
        return self.polygon.area();
    }

    pub fn construction(&self) -> Rc<Construction>{
        Rc::clone(&self.construction)
    }


}



