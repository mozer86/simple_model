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

use derive::SimpleInputOutput;
use crate::Float;


/// An infiltration rate for a `Space`
#[derive(Clone, SimpleInputOutput)]
pub enum Infiltration {
    
    /// A contant infiltration, specified in `m3/s`
    Constant(Float),    

    /// Sets the infiltration to the `DesignFlowRate` values using the 
    /// default from BLAST as described in the EnergyPlus' Input Output reference
    Blast(Float),

    /// Sets the infiltration to the `DesignFlowRate` values using the 
    /// default from DOE-2 as described in the EnergyPlus' Input Output reference
    Doe2(Float),

    /// Sets the infiltration to the `DesignFlowRate` values using an 
    /// arbitrary set of values. This option is based on EnergyPlus'
    /// object of the same name.
    /// 
    /// 
    /// The flow $\phi$ (in $m^3/s$) is calculated from the parameters $A$, $B$, $C$, $D$ and 
    /// $\phi_{design}$ as follows:
    /// 
    /// $$ \phi = \phi_{design} (A + B|T_{space} - T_{outside}| + C\times W_{speed} + D\times W^2_{speed})$$
    /// 
    /// The inputs to this object are $A$, $B$, $C$, $D$, $\phi_{design}$ .
    DesignFlowRate(Float, Float, Float, Float, Float),

    /// Sets the infiltration based on `EffectiveLeakageArea` as 
    /// described in the EnergyPlus' Input Output reference.
    ///     
    /// The infiltration rate—in $m^3/s$—is calculated based on the 
    /// following equation: 
    /// 
    /// $$ \phi = \frac{A_L}{1000} \sqrt{C_s \Delta T + C_w W^2_{speed}}$$
    /// 
    /// where:
    /// * $A_L$ is the effecctive air leakage in $cm^2$ @ 4Pa
    /// * $C_s$ is the coefficient for stack induced infiltration
    /// * $C_w$ is the coefficient for wind induced infiltration
    /// 
    /// **The only input to this object is the effecctive air leakage, $A_L$, in $cm^2$ @ 4Pa**. 
    /// The other parameters—$C_s$ and $C_w$—are derived based 
    /// on the required `Building` object associated with the `Space` that owns 
    /// this `Infiltration`. For this to work, the associated `Building` needs
    /// to have been assigned the fields `n_storeys` and a `shelter_class`
    /// (which allow calculating $C_s$ and $C_w$) OR the properties of 
    /// `stack_coefficient` (i.e., $C_s$) and `wind_coefficient` (i.e., $C_w$).
    ///
    /// > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
    /// > of 3 storeys or less.
    /// 
    /// ### Example 
    /// 
    /// ```rs
    /// Building {
    ///     name: "Main campus", 
    ///     n_storeys: 2,
    ///     shelter_class: ShelterClass::Urban
    /// }
    ///
    /// Space {
    ///     name: "Bedroom",
    ///     volume: 72.,
    ///     building: "Main campus", 
    ///     infiltration : Infiltration::EffectiveAirLeakageArea(300)
    /// }
    /// ```
    EffectiveAirLeakageArea(Float),

    
    


    // FlowCoefficient...?
}

