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
use derive::SimpleSimulationStateBehaviour;


pub type StateElementField = RefCell<Option<usize>>;


/// The idea is to have a cheap-to-clone (or copy?) structure
#[derive(Debug, Copy, Clone, PartialEq, SimpleSimulationStateBehaviour)]
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
    /// Contains the index of the HVAC in the building's vector,
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
    // /// Represents the Brightness of a space.
    // ///
    // /// This perception is a placeholder. I need to
    // /// understand better what makes a space look "bright"
    // /// and how that relates to its attractiveness and
    // /// cleanliness and all.
    // ///
    // /// **This is written as a perception for now,
    // /// but it should be a physical quantity**
    // #[physical]
    // SpaceBrightness(usize),

    
    /// The convective heat transfer coefficient
    /// at the front of a surface
    #[physical]
    SurfaceFrontConvectionCoefficient(usize),

    /// The convective heat transfer coefficient
    /// at the back of a surface
    #[physical]
    SurfaceBackConvectionCoefficient(usize),

    /// The convective heat flow 
    /// at the front of a surface
    #[physical]
    SurfaceFrontConvectiveHeatFlow(usize),

    /// The convective heat flow 
    /// at the back of a surface
    #[physical]
    SurfaceBackConvectiveHeatFlow(usize),

    /// Incident solar irradiance at the front
    #[physical]
    SurfaceFrontSolarIrradiance(usize),

    /// Incident solar irradiance at the back
    #[physical]
    SurfaceBackSolarIrradiance(usize),

    /// Incident Infrared irradiance at the front
    #[physical]
    SurfaceFrontIRIrradiance(usize),

    /// Incident Infrared irradiance at the back
    #[physical]
    SurfaceBackIRIrradiance(usize),

    /// The convective heat transfer coefficient
    /// at the front of a surface
    #[physical]
    FenestrationFrontConvectionCoefficient(usize),

    /// The convective heat transfer coefficient
    /// at the back of a surface
    #[physical]
    FenestrationBackConvectionCoefficient(usize),

    /// The convective heat flow 
    /// at the front of a surface
    #[physical]
    FenestrationFrontConvectiveHeatFlow(usize),

    /// The convective heat flow 
    /// at the back of a surface
    #[physical]
    FenestrationBackConvectiveHeatFlow(usize),

    /// Incident solar irradiance at the front
    #[physical]
    FenestrationFrontSolarIrradiance(usize),

    /// Incident solar irradiance at the back
    #[physical]
    FenestrationBackSolarIrradiance(usize),

    /// Incident Infrared irradiance at the front
    #[physical]
    FenestrationFrontIRIrradiance(usize),

    /// Incident Infrared irradiance at the back
    #[physical]
    FenestrationBackIRIrradiance(usize),

    
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
    // /// Represents the loudness in a certain space
    // ///
    // /// **This is written as a perception for now,
    // /// but it should be a physical quantity**
    // #[physical]
    // SpaceLoudness(usize),
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
