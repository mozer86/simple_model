use building_state_macro::BuildingObjectBehaviour;
use geometry3d::polygon3d::Polygon3D;
use std::rc::Rc;

use crate::boundary::*;
use crate::building::Building;
use crate::construction::Construction;
//use crate::fenestration::*;
use crate::simulation_state::SimulationState;

/// A fixed surface in the building (or surroundings). This can be of
/// any Construction, transparent or not.
#[derive(Clone, BuildingObjectBehaviour)]
pub struct Surface {
    /// The name of the surface
    pub name: String,

    /// The Polygon3D that represents
    /// the dimensions and size of the Surface
    pub polygon: Polygon3D,

    /// The index of the construction in the Building's
    /// Construction array    
    pub construction: Rc<Construction>,

    /// A reference to the Boundary in front of the Surface
    front_boundary: Option<Boundary>,

    /// A reference to the Boundary in back of the Surface
    back_boundary: Option<Boundary>,

    index: Option<usize>,

    /* STATE */
    #[state]
    first_node_temperature: Option<usize>,

    #[state]
    last_node_temperature: Option<usize>,
}

/// A surface in the Building, separating two spaces,
/// or a space and the exterior, or exterior and exterior
impl Surface {
    /// Returns the area of the [`Surface`] (calculated
    /// based on the [`Polygon3D`] that represents it)
    pub fn area(&self) -> f64 {
        self.polygon.area()
    }
}

impl Building {
    /* SURFACE */

    /// Creates a new Surface
    pub fn add_surface(&mut self, mut surface: Surface) -> &Surface {
        surface.index = Some(self.surfaces.len());
        self.surfaces.push(surface);
        self.surfaces.last().unwrap()
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
        assert!(surf.first_node_temperature.is_none());
        assert!(surf.first_node_temperature_index().is_none());
        assert!(surf.last_node_temperature.is_none());
        assert!(surf.last_node_temperature_index().is_none());

        surf.set_front_boundary(Boundary::Ground);
        surf.set_back_boundary(Boundary::Space(1));
        surf.set_first_node_temperature_index(31);
        surf.set_last_node_temperature_index(39);

        assert!(surf.front_boundary.is_some());
        if let Ok(Boundary::Ground) = surf.front_boundary() {}
        assert!(surf.back_boundary.is_some());
        if let Ok(Boundary::Space(i)) = surf.back_boundary() {
            assert_eq!(i, 1);
        }

        assert!(surf.first_node_temperature.is_some());
        assert_eq!(surf.first_node_temperature_index(), Some(31));
        assert!(surf.last_node_temperature.is_some());
        assert_eq!(surf.last_node_temperature_index(), Some(39));

        // match s0.front_boundary() {
        //     Boundary::None => {}
        //     _ => assert!(false),
        // };
        // match s0.back_boundary() {
        //     Boundary::None => {}
        //     _ => assert!(false),
        // }
        // assert!(s0.is_full().is_err());
        // assert!(s0.area().is_err());

        // // set, get construction
        // let construction_index = 9872;
        // s0.set_construction(construction_index);
        // if let Some(i) = s0.get_construction_index() {
        //     assert_eq!(i, construction_index);
        // } else {
        //     assert!(false);
        // }

        // // set,get front boundary
        // s0.set_front_boundary(Boundary::Ground).unwrap();
        // match s0.front_boundary() {
        //     Boundary::Ground => {}
        //     _ => assert!(false),
        // };
        // s0.set_back_boundary(Boundary::Ground).unwrap();
        // match s0.back_boundary() {
        //     Boundary::Ground => {}
        //     _ => assert!(false),
        // };

        // // polygon still missing
        // assert!(s0.is_full().is_err());
    }
}
