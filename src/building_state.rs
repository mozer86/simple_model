use crate::heating_cooling::HeatingCoolingState;

pub type BuildingState = Vec<BuildingStateElement>;

pub fn find_in_state(state: &BuildingState, element: BuildingStateElement )->Option<usize>{
    for i in 0..state.len(){
        if state[i] == element{
            return Some(i)
        }
    }
    None
}

// The idea is to have a cheap-to-clone (or copy?) structure
#[derive(Copy,Clone, PartialEq)]
pub enum BuildingStateElement{
    
    /* OPERATION AND OCCUPATION */

    /// Represents how open is a fenestration.
    /// Contains the Index of fenestration, and its open fraction
    FenestrationOpenFraction(usize,f64),

    /// Represents the heating/cooling state on a space
    /// One heater is allowed per space. So, it contains
    /// the Space index and the fraction of power
    SpaceHeatingCooling(usize, HeatingCoolingState),

    /* SOLAR */
    
    // Space
    //SpaceTotalSolarHeatGain(usize,f64),
    //SpaceDirectSolarHeatGain(usize,f64),
    //SpaceDiffuseSolarHeatGain(usize,f64),

    // Surface
    //SurfaceFrontTotalSolarIrradiance(usize,f64),
    //SurfaceBackTotalSolarIrradiance(usize,f64),
    //SurfaceFrontDirectSolarIrradiance(usize,f64),
    //SurfaceBackDirectSolarIrradiance(usize,f64),
    //SurfaceFrontDiffuseSolarIrradiance(usize,f64),
    //SurfaceBackDiffuseSolarIrradiance(usize,f64),

    /* THERMAL */

    /// Space Air Temperature... The elements
    /// are the index of the Space in the Building mode
    /// and the temperature
    SpaceDryBulbTemperature(usize,f64),
        
  
    /// Surface inner temperature (f64) of Surface's (usize) node usize
    /// I.e. the order is (Surface Index, Node index, Temperature).    
    SurfaceNodeTemperature(usize,usize,f64),

    // Fenestration

    // Shading

    // 
}

impl BuildingStateElement {
    
    pub fn to_string(&self)->String{
        match self{
            BuildingStateElement::SpaceDryBulbTemperature(space_index,_) => {
                format!("Space {} Dry Bulb Temperature [C]", space_index)
            },
            BuildingStateElement::SurfaceNodeTemperature(space_index,node_index,_)=>{
                format!("Surface {} - Node {} Temperature [C]", space_index, node_index)
            },
            BuildingStateElement::FenestrationOpenFraction(fenestration_index,_)=>{
                format!("Fenestration {} - OpenFraction []", fenestration_index)
            },
            BuildingStateElement::SpaceHeatingCooling(space_index,_)=>{
                format!("Space {} - Heating/Cooling Power Fraction", space_index)
            }
        }
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing{
    use super::*;

    #[test]
    fn test_compare(){
        let i = 2;
        let v = 2.1231;
        let a = BuildingStateElement::SpaceDryBulbTemperature(i,v);

        assert!(a == BuildingStateElement::SpaceDryBulbTemperature(i,v));
        assert!(a != BuildingStateElement::SpaceDryBulbTemperature(2*i,v));
        assert!(a != BuildingStateElement::SpaceDryBulbTemperature(i,2.*v));
        assert!(a != BuildingStateElement::SurfaceNodeTemperature(i,2,v));

    }

    #[test]
    fn test_find(){

        let mut state : BuildingState = Vec::new();
        for i in 0..23{
            state.push(
                BuildingStateElement::SpaceDryBulbTemperature(i,2.0 * i as f64)
            )
        }

        let i = 12;
        if let Some(index) = find_in_state(&state, BuildingStateElement::SpaceDryBulbTemperature(i,2.0 * i as f64)){
            assert_eq!(index,i);
        }else{
            assert!(false)
        }

        let i = 8;
        if let Some(index) = find_in_state(&state, BuildingStateElement::SpaceDryBulbTemperature(i,2.0 * i as f64)){
            assert_eq!(index,i);
        }else{
            assert!(false)
        }

    }

}
