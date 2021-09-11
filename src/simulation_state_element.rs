use building_state_macro::SimulationStateBehaviour;

/// The idea is to have a cheap-to-clone (or copy?) structure
#[derive(Debug, Copy, Clone, PartialEq, SimulationStateBehaviour)]
pub enum SimulationStateElement {
    /* PERSONAL ELEMENTS */
    /// The amount of clothing the person is using,
    /// in Clo value
    #[personal]
    Clothing(f64),

    /* OPERATION AND OCCUPATION */
    /// Represents how open is a fenestration.
    /// Contains the Index of fenestration, and its open fraction
    #[operational]
    FenestrationOpenFraction(usize, f64),

    /// Represents the heating/cooling energy consumption of a Heating/Cooling system,
    /// in Watts
    ///
    /// Contains the index of the HeaterCooler in the building's vector,
    /// and the power.        
    #[operational]
    HeatingCoolingPowerConsumption(usize, f64),

    /// Represents the power being consumed by
    /// a Luminaire object, in Watts (luminaire index, power)
    #[operational]
    LuminairePowerConsumption(usize, f64),

    /* SOLAR */
    // Space
    //SpaceTotalSolarHeatGain(usize,f64),
    //SpaceDirectSolarHeatGain(usize,f64),
    //SpaceDiffuseSolarHeatGain(usize,f64),
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
    SpaceBrightness(usize, f64),

    // Surface
    //SurfaceFrontTotalSolarIrradiance(usize,f64),
    //SurfaceBackTotalSolarIrradiance(usize,f64),
    //SurfaceFrontDirectSolarIrradiance(usize,f64),
    //SurfaceBackDirectSolarIrradiance(usize,f64),
    //SurfaceFrontDiffuseSolarIrradiance(usize,f64),
    //SurfaceBackDiffuseSolarIrradiance(usize,f64),

    /* THERMAL */
    /// Space Air Temperature in C... The elements
    /// are the index of the Space in the Building mode
    /// and the temperature
    #[physical]
    SpaceDryBulbTemperature(usize, f64),

    /// The volume of air that is entering the space in
    /// an uncontrolled way. In m3/s
    #[physical]
    SpaceInfiltrationVolume(usize, f64),

    /// The temperature of air that is entering the space in
    /// an uncontrolled way. In C
    #[physical]
    SpaceInfiltrationTemperature(usize, f64),

    /// The volume of air that is entering the space in
    /// a controlled way. In m3/s
    #[physical]
    SpaceVentilationVolume(usize, f64),

    /// The temperature of air that is entering the space in
    /// a controlled way. In C
    #[physical]
    SpaceVentilationTemperature(usize, f64),

    /// The volume of air that is moving from one space to another in
    /// a controlled way. In m3/s
    #[physical]
    SpaceAirExchangeVolume(usize, usize, f64),

    /// Temperature (f64) of Surface's (usize) node (usize)
    /// I.e. the order is (Surface Index, Node index, Temperature).    
    #[physical]
    SurfaceNodeTemperature(usize, usize, f64),

    /// Temperature (f64) of Fenestration's (usize) node (usize)
    /// I.e. the order is (Surface Index, Node index, Temperature).    
    #[physical]
    FenestrationNodeTemperature(usize, usize, f64),

    // Temperature (f64) of Fenestation's (usize) node usize
    // I.e. the order is (Surface Index, Node index, Temperature).
    //FenestrationNodeTemperature(usize,usize,f64),

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
    SpaceLoudness(usize, f64),
}

impl SimulationStateElement {
    pub fn safe_get_value(&self, pattern: Self) -> f64 {
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

    /// Transforms a StateElement into a String
    pub fn to_string(&self) -> String {
        match self {
            // Individual ones
            Self::Clothing(_) => format!("Clothing"),

            // Operational ones
            Self::FenestrationOpenFraction(fenestration_index, _) => {
                format!("Fenestration {} - OpenFraction [-]", fenestration_index)
            }
            Self::HeatingCoolingPowerConsumption(space_index, _) => {
                format!("Heating/Cooling {} - Power Consumption [W]", space_index)
            }
            Self::LuminairePowerConsumption(space_index, _) => {
                format!("Luminaire {} - Lighting Power Consumption [W]", space_index)
            }
            Self::SpaceInfiltrationVolume(space_index, _) => {
                format!("Space {} - Infiltration Volume [m3/s]", space_index)
            }
            Self::SpaceInfiltrationTemperature(space_index, _) => {
                format!("Space {} - Infiltration [C]", space_index)
            }
            Self::SpaceVentilationVolume(space_index, _) => {
                format!("Space {} - Ventilation Volume [m3/s]", space_index)
            }
            Self::SpaceVentilationTemperature(space_index, _) => {
                format!("Space {} - Ventilation Temperature [C]", space_index)
            }
            Self::SpaceAirExchangeVolume(origin, target, _) => {
                format!("Space {} to Space {} - Air Exchange [m3/s]", origin, target)
            }

            // Physical ones
            Self::SpaceDryBulbTemperature(space_index, _) => {
                format!("Space {} Dry Bulb Temperature [C]", space_index)
            }

            Self::SurfaceNodeTemperature(surface_index, node_index, _) => {
                format!(
                    "Surface {} - Node {} Temperature [C]",
                    surface_index, node_index
                )
            }
            Self::FenestrationNodeTemperature(fen_index, node_index, _) => {
                format!(
                    "Fenestration {} - Node {} Temperature [C]",
                    fen_index, node_index
                )
            }
            Self::SpaceBrightness(space_index, _) => {
                format!("Space {} - Brightness", space_index)
            }
            Self::SpaceLoudness(space_index, _) => {
                format!("Space {} - Loudness", space_index)
            }
        }
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_differ_only_in_value() {
        // Equals
        let a = SimulationStateElement::Clothing(2.0);
        let b = SimulationStateElement::Clothing(2.0);
        assert!(a.differ_only_in_value(b).is_ok());

        // Same variant, Different value (same content)
        let a = SimulationStateElement::Clothing(2.0);
        let b = SimulationStateElement::Clothing(1.0);
        assert!(a.differ_only_in_value(b).is_ok());

        // Different variant
        let a = SimulationStateElement::Clothing(2.0);
        let b = SimulationStateElement::SpaceDryBulbTemperature(2, 1.0);
        assert!(a.differ_only_in_value(b).is_err());

        // Same variant, different content
        let a = SimulationStateElement::SpaceDryBulbTemperature(3, 2.0);
        let b = SimulationStateElement::SpaceDryBulbTemperature(2, 1.0);
        assert!(a.differ_only_in_value(b).is_err());

        // Same variant, same content, different value
        let a = SimulationStateElement::SpaceDryBulbTemperature(2, 2.0);
        let b = SimulationStateElement::SpaceDryBulbTemperature(2, 1.0);
        assert!(a.differ_only_in_value(b).is_ok());
    }

    #[test]
    fn test_compare() {
        let i = 2;
        let v = 2.1231;
        let a = SimulationStateElement::SpaceDryBulbTemperature(i, v);

        assert!(a == SimulationStateElement::SpaceDryBulbTemperature(i, v));
        assert!(a != SimulationStateElement::SpaceDryBulbTemperature(2 * i, v));
        assert!(a != SimulationStateElement::SpaceDryBulbTemperature(i, 2. * v));
        assert!(a != SimulationStateElement::SurfaceNodeTemperature(i, 2, v));
    }

    #[test]
    fn test_get_value() {
        let v = 2.1231;
        let temp = SimulationStateElement::SpaceDryBulbTemperature(0, v);
        let b = SimulationStateElement::SurfaceNodeTemperature(0, 1, v);

        assert_eq!(v, temp.get_value());
        assert_eq!(v, b.get_value());
    }

    #[test]
    fn test_classify() {
        // Physical
        let e = SimulationStateElement::SpaceDryBulbTemperature(2, 2.);
        assert!(e.is_physical());
        assert!(!e.is_operational());
        assert!(!e.is_personal());

        // Individual
        let e = SimulationStateElement::Clothing(2.);
        assert!(!e.is_physical());
        assert!(!e.is_operational());
        assert!(e.is_personal());

        // Operational
        let e = SimulationStateElement::HeatingCoolingPowerConsumption(2, 0.0);
        assert!(!e.is_physical());
        assert!(e.is_operational());
        assert!(!e.is_personal());
    }
}
