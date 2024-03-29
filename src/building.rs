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
use crate::model::SimpleModel;
use crate::Float;
use derive::ObjectIO;

use std::rc::Rc;

#[derive(Clone, ObjectIO)]
pub enum ShelterClass {
    /// No obstructions or local shielding
    NoObstructions,

    /// Typical shelter for an isolated rural house
    IsolatedRural,

    /// Typical shelter caused by other buildings across the street
    Urban,

    /// Typical shelter for urban buildings on larger lots
    LargeLotUrban,

    /// Typical shelter produced by buildings that are immediately adjacent.
    SmallLotUrban,
}

/// This object is utilized to group `Space` objects together for
/// metering and/or shared values. For example, the number of storeys
/// and the `ShelterClass` will help defining the `Infiltrations`
#[derive(ObjectIO, Clone)]
pub struct Building {
    /// The name of the Building
    pub name: String,

    /// The index of the building in the SimpleModel's buildings array
    index: Option<usize>,

    /// The number of storeys of this building.
    ///
    /// This value use used by the `AirFlow` module when a `Space` associated
    /// to this `Building` has been assigned an `EffectiveAirLeakageArea`
    /// infiltration. This value is required for calculating the Stack
    /// Coefficient ($C_s$) and the Wind Coefficient ($C_w$) of the
    /// `EffectiveAirLeakageArea` infiltration. $C_s$ and $C_w$ can be inputed
    /// directly by assigning values to the `stack_coefficient` and
    /// `wind_coefficient` fields, in which case the `n_storeys` field will
    /// be ignored.
    n_storeys: Option<usize>,

    /// The `ShelterClass` of this building.
    ///
    /// This value use used by the `AirFlow` module when a `Space` associated
    /// to this `Building` has been assigned an `EffectiveAirLeakageArea`
    /// infiltration. This value is required for calculating the Wind
    /// Coefficient ($C_s$) of the
    /// `EffectiveAirLeakageArea` infiltration.  $C_w$ can be inputed
    /// directly by assigning values to the `wind_coefficient` field, in
    /// which case the `shelter_class` field will be ignored.
    shelter_class: Option<ShelterClass>,

    /// The stack coefficient of this building, used for
    /// calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
    /// infiltration option.
    ///
    /// If not given, the number of storeys will be used for getting
    /// this values (based on EnergyPlus' Engineering Reference).
    ///
    /// > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
    /// > of 3 storeys or less.
    stack_coefficient: Option<Float>,

    /// The wind coefficient of this building, used for
    /// calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
    /// infiltration option.
    ///
    /// If not given, the number of storeys will be used for getting
    /// this values (based on EnergyPlus' Engineering Reference).
    ///
    /// > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
    /// > of 3 storeys or less.
    wind_coefficient: Option<Float>,
}

impl SimpleModel {
    /// Adds a [`Building`] to the [`SimpleModel`]
    pub fn add_building(&mut self, mut add: Building) -> Rc<Building> {
        add.set_index(self.buildings.len());
        let add = Rc::new(add);
        self.buildings.push(Rc::clone(&add));
        add
    }
}
