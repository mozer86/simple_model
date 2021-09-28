use std::rc::Rc;
use building_state_macro::SimpleInputOutput;
use crate::scanner::{Scanner, TokenType};
use crate::space::Space;
use crate::model::SimpleModel;


/// Represents the boundary of a `Surface`
/// 
/// By default (i.e., if no boundary is assigned to a Surface), 
/// the boundary will be assumed to be outside. 
#[derive(Clone, SimpleInputOutput)]
pub enum Boundary {
    
    /// The Surface is in contact with the Ground
    Ground,

    /// The Surface leads to another surface
    Space(Rc<Space>),

}




/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;


    #[test]
    fn test_boundary_from_bytes(){
        
        /* Ground */
        let mut building = SimpleModel::new("the building".to_string());
        let bytes = b" ::Ground,";
        let bound = Boundary::from_bytes(bytes, &mut building).unwrap();
        if let Boundary::Ground = bound{
            assert!(true)
        }else{
            assert!(false)
        }

        /* SPACE BY NAME */
        let mut building = SimpleModel::new("the building".to_string());
        let space = Space::new("the space".to_string());
        let space = building.add_space(space);

        let bytes = b" ::Space(\"the space\"),";
        let bound = Boundary::from_bytes(bytes, &mut building).unwrap();
        if let Boundary::Space(s) = &bound{
            assert!(Rc::ptr_eq(s, &space));
        }else{
            assert!(false)
        }

        /* SPACE ANONYMOUS */
        let mut building = SimpleModel::new("the building".to_string());        
        let bytes = b" ::Space(Space{
            name: \"Some Space\"
        }),";
        let bound = Boundary::from_bytes(bytes, &mut building).unwrap();
        if let Boundary::Space(s) = &bound{
            assert_eq!(s.name, "Some Space");
        }else{
            assert!(false)
        }
        

    }
}
