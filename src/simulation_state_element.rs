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

use std::cell::RefCell;
use building_state_macro::SimulationStateBehaviour;


pub type StateElementField = RefCell<Option<usize>>;


/// The idea is to have a cheap-to-clone (or copy?) structure
#[derive(Debug, Copy, Clone, PartialEq, SimulationStateBehaviour)]
pub enum SimulationStateElement {
    /* PERSONAL ELEMENTS */
    /// The amount of clothing the person is using,
    /// in Clo value
    #[personal]
    Clothing,

    /* OPERATION AND OCCUPATION */
    /// Represents how open is a fenestration.
    /// Contains the Index of fenestration, and its open fraction
    #[operational]
    FenestrationOpenFraction(usize),

    /// Represents the heating/cooling energy consumption of a Heating/Cooling system,
    /// in Watts
    ///
    /// Contains the index of the HeaterCooler in the building's vector,
    /// and the power.        
    #[operational]
    HeatingCoolingPowerConsumption(usize),

    /// Represents the power being consumed by
    /// a Luminaire object, in Watts (luminaire index, power)
    #[operational]
    LuminairePowerConsumption(usize),

    /* SOLAR */
    // Space
    //SpaceTotalSolarHeatGain(usize),
    //SpaceDirectSolarHeatGain(usize),
    //SpaceDiffuseSolarHeatGain(usize),
    /// Represents the Brightness of a space.
    ///
    /// This perception is a placeholder. I need to
    /// understand better what makes a space look "bright"
    /// and how that relates to its attractiveness and
    /// cleanliness and all.
    ///
    /// **This is written as a perception for now,
    /// but it should be a physical quantity**
    #[physical]
    SpaceBrightness(usize),

    // Surface
    //SurfaceFrontTotalSolarIrradiance(usize),
    //SurfaceBackTotalSolarIrradiance(usize),
    //SurfaceFrontDirectSolarIrradiance(usize),
    //SurfaceBackDirectSolarIrradiance(usize),
    //SurfaceFrontDiffuseSolarIrradiance(usize),
    //SurfaceBackDiffuseSolarIrradiance(usize),

    /* THERMAL */
    /// Space Air Temperature in C... The elements
    /// are the index of the Space in the Building mode
    /// and the temperature
    #[physical]
    SpaceDryBulbTemperature(usize),

    /// The volume of air that is entering the space in
    /// an uncontrolled way. In m3/s
    #[physical]
    SpaceInfiltrationVolume(usize),

    /// The temperature of air that is entering the space in
    /// an uncontrolled way. In C
    #[physical]
    SpaceInfiltrationTemperature(usize),

    /// The volume of air that is entering the space in
    /// a controlled way. In m3/s
    #[physical]
    SpaceVentilationVolume(usize),

    /// The temperature of air that is entering the space in
    /// a controlled way. In C
    #[physical]
    SpaceVentilationTemperature(usize),

    /// The volume of air that is moving from one space to another in
    /// a controlled way. In m3/s
    #[physical]
    SpaceAirExchangeVolume(usize, usize),

    /// Temperature (Float) of Surface's (usize) node (usize)
    /// I.e. the order is (Surface Index, Node index, Temperature).    
    #[physical]
    SurfaceNodeTemperature(usize, usize),

    /// Temperature (Float) of Fenestration's (usize) node (usize)
    /// I.e. the order is (Surface Index, Node index, Temperature).    
    #[physical]
    FenestrationNodeTemperature(usize, usize),

    // Temperature (Float) of Fenestation's (usize) node usize
    // I.e. the order is (Surface Index, Node index, Temperature).
    //FenestrationNodeTemperature(usize,usize),

    // Fenestration

    // Shading

    //

    /* ACOUSTIC */
    // Space
    /// Represents the loudness in a certain space
    ///
    /// **This is written as a perception for now,
    /// but it should be a physical quantity**
    #[physical]
    SpaceLoudness(usize),
}

impl SimulationStateElement {
    /*
    pub fn safe_get_value(&self, pattern: Self) -> Float {
        match self.differ_only_in_value(pattern) {
            Ok(()) => self.get_value(),
            Err(e) => {
                panic!(
                    "Corrupt Simulation State : '{}' Looking for pattern '{:?}', found '{:?}'",
                    e, pattern, self
                )
            }
        }
    }
    */

    // /// Transforms a StateElement into a String
    // pub fn to_string(&self) -> String {
    //     match self {
    //         // Individual ones
    //         Self::Clothing(_) => format!("Clothing"),

    //         // Operational ones
    //         Self::FenestrationOpenFraction(fenestration_index, _) => {
    //             format!("Fenestration {} - OpenFraction [-]", fenestration_index)
    //         }
    //         Self::HeatingCoolingPowerConsumption(space_index, _) => {
    //             format!("Heating/Cooling {} - Power Consumption [W]", space_index)
    //         }
    //         Self::LuminairePowerConsumption(space_index, _) => {
    //             format!("Luminaire {} - Lighting Power Consumption [W]", space_index)
    //         }
    //         Self::SpaceInfiltrationVolume(space_index, _) => {
    //             format!("Space {} - Infiltration Volume [m3/s]", space_index)
    //         }
    //         Self::SpaceInfiltrationTemperature(space_index, _) => {
    //             format!("Space {} - Infiltration [C]", space_index)
    //         }
    //         Self::SpaceVentilationVolume(space_index, _) => {
    //             format!("Space {} - Ventilation Volume [m3/s]", space_index)
    //         }
    //         Self::SpaceVentilationTemperature(space_index, _) => {
    //             format!("Space {} - Ventilation Temperature [C]", space_index)
    //         }
    //         Self::SpaceAirExchangeVolume(origin, target, _) => {
    //             format!("Space {} to Space {} - Air Exchange [m3/s]", origin, target)
    //         }

    //         // Physical ones
    //         Self::SpaceDryBulbTemperature(space_index, _) => {
    //             format!("Space {} Dry Bulb Temperature [C]", space_index)
    //         }

    //         Self::SurfaceNodeTemperature(surface_index, node_index, _) => {
    //             format!(
    //                 "Surface {} - Node {} Temperature [C]",
    //                 surface_index, node_index
    //             )
    //         }
    //         Self::FenestrationNodeTemperature(fen_index, node_index, _) => {
    //             format!(
    //                 "Fenestration {} - Node {} Temperature [C]",
    //                 fen_index, node_index
    //             )
    //         }
    //         Self::SpaceBrightness(space_index, _) => {
    //             format!("Space {} - Brightness", space_index)
    //         }
    //         Self::SpaceLoudness(space_index, _) => {
    //             format!("Space {} - Loudness", space_index)
    //         }
    //     }
    // }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    

    #[test]
    fn test_compare() {
        let i = 2;
        let a = SimulationStateElement::SpaceDryBulbTemperature(i);

        assert!(a == SimulationStateElement::SpaceDryBulbTemperature(i));        
        assert!(a != SimulationStateElement::SpaceDryBulbTemperature(2 * i));
        assert!(a != SimulationStateElement::SurfaceNodeTemperature(i, 2));
    }

    
    #[test]
    fn test_classify() {
        // Physical
        let e = SimulationStateElement::SpaceDryBulbTemperature(2);
        assert!(e.is_physical());
        assert!(!e.is_operational());
        assert!(!e.is_personal());

        // Individual
        let e = SimulationStateElement::Clothing;
        assert!(!e.is_physical());
        assert!(!e.is_operational());
        assert!(e.is_personal());

        // Operational
        let e = SimulationStateElement::HeatingCoolingPowerConsumption(2);
        assert!(!e.is_physical());
        assert!(e.is_operational());
        assert!(!e.is_personal());
    }
}
