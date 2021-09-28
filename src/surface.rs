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
use crate::Float;

use building_state_macro::SimpleObjectBehaviour;
use geometry3d::polygon3d::Polygon3D;
use std::rc::Rc;
use crate::boundary::*;
use crate::construction::Construction;
use crate::simulation_state_element::StateElementField;
use crate::simulation_state::SimulationState;

/// A fixed surface in the building (or surroundings). This can be of
/// any Construction, transparent or not.
#[derive(SimpleObjectBehaviour)]
pub struct Surface {
    /// The name of the surface
    pub name: String,

    /// The Polygon3D that represents
    /// the dimensions and size of the Surface
    pub polygon: Polygon3D,

    /// The index of the construction in the SimpleModel's
    /// Construction array    
    pub construction: Rc<Construction>,

    /// A reference to the Boundary in front of the Surface
    front_boundary: Option<Boundary>,

    /// A reference to the Boundary in back of the Surface
    back_boundary: Option<Boundary>,
    
    /* STATE */
    #[state]
    first_node_temperature: StateElementField,

    #[state]
    last_node_temperature: StateElementField,
}

/// A surface in the SimpleModel, separating two spaces,
/// or a space and the exterior, or exterior and exterior
impl Surface {
    /// Returns the area of the [`Surface`] (calculated
    /// based on the [`Polygon3D`] that represents it)
    pub fn area(&self) -> Float {
        self.polygon.area()
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use geometry3d::loop3d::Loop3D;
    use geometry3d::point3d::Point3D;
    use geometry3d::polygon3d::Polygon3D;

    #[test]
    fn test_surface_basic() {
        let construction = Rc::new(Construction::new("the construction".to_string()));
        let mut outer = Loop3D::new();
        outer.push(Point3D::new(0., 0., 0.)).unwrap();
        outer.push(Point3D::new(2., 0., 0.)).unwrap();
        outer.push(Point3D::new(2., 2., 0.)).unwrap();
        outer.push(Point3D::new(0., 2., 0.)).unwrap();
        outer.close().unwrap();

        let polygon = Polygon3D::new(outer).unwrap();

        let surf_name = "Some surface".to_string();
        let mut surf = Surface::new(surf_name.clone(), polygon, construction);

        assert!(surf.front_boundary.is_none());
        assert!(surf.front_boundary().is_err());
        assert!(surf.back_boundary.is_none());
        assert!(surf.back_boundary().is_err());
        assert!(surf.first_node_temperature.borrow().is_none());
        assert!(surf.first_node_temperature_index().is_none());
        assert!(surf.last_node_temperature.borrow().is_none());
        assert!(surf.last_node_temperature_index().is_none());

        surf.set_front_boundary(Boundary::Ground);
        surf.set_first_node_temperature_index(31);
        surf.set_last_node_temperature_index(39);

        assert!(surf.front_boundary.is_some());
        if let Ok(Boundary::Ground) = surf.front_boundary() {}
        assert!(surf.back_boundary.is_none());
        

        assert!(surf.first_node_temperature.borrow().is_some());
        assert_eq!(surf.first_node_temperature_index(), Some(31));
        assert!(surf.last_node_temperature.borrow().is_some());
        assert_eq!(surf.last_node_temperature_index(), Some(39));

    }
}
