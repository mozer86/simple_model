/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::rc::Rc;
use derive::SimpleInputOutput;

use crate::space::Space;



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

    use crate::model::SimpleModel;
    #[test]
    fn test_boundary_from_bytes(){
        
        /* Ground */
        let mut building = SimpleModel::new("the building".to_string());
        let bytes = b" ::Ground,";
        let bound = Boundary::from_bytes(1, bytes, &mut building).unwrap();
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
        let bound = Boundary::from_bytes(1, bytes, &mut building).unwrap();
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
        let bound = Boundary::from_bytes(1, bytes, &mut building).unwrap();
        if let Boundary::Space(s) = &bound{
            assert_eq!(s.name, "Some Space");
        }else{
            assert!(false)
        }
        

    }
}
