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

use crate::hvac::*;
use building_state_macro::{SimpleObjectBehaviour, ModelGetterSetter};
use std::rc::Rc;

use crate::construction::Construction;
use crate::fenestration::Fenestration;
use crate::luminaire::Luminaire;
use crate::material::Material;
use crate::space::Space;
use crate::substance::Substance;
use crate::surface::Surface;


#[derive(Default, SimpleObjectBehaviour, ModelGetterSetter)]
pub struct SimpleModel {
    /// The name of the building
    pub name: String,

    // materiality
    pub substances: Vec<Rc<Substance>>,
    pub materials: Vec<Rc<Material>>,
    pub constructions: Vec<Rc<Construction>>,

    // geometry
    pub surfaces: Vec<Rc<Surface>>,
    pub spaces: Vec<Rc<Space>>,

    /// The windows and doors in the surface    
    pub fenestrations: Vec<Rc<Fenestration>>,

    /// The Heating/Cooling devices in the space
    pub hvacs: Vec<Rc<dyn HVAC>>,

    /// Luminaires
    pub luminaires: Vec<Rc<Luminaire>>,
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::hvac::electric_heater::ElectricHeater;
    

    #[test]
    fn building_substance() {
        let mut building = SimpleModel::new("Test Model".to_string());

        let subs_name = "Substance 0".to_string();
        let substance = Substance::new(subs_name.clone());

        let s0 = building.add_substance(substance);

        let s = &building.substances[0];        
        assert_eq!(subs_name, s.name);
        assert_eq!(subs_name, s0.name);
        assert_eq!(*s0.index().unwrap(), 0);
    }

    #[test]
    fn building_hvac() {
        let mut building = SimpleModel::new("Test Model".to_string());

        let heater_name = "Heater".to_string();
        let heater = ElectricHeater::new(heater_name.clone());

        let h0 = building.add_hvac(Rc::new(heater));

        let h = cast_hvac::<ElectricHeater>(&*h0).unwrap();        
        assert_eq!(heater_name, h.name);        
        assert_eq!(*h.index().unwrap(), 0);

        let h = cast_hvac::<ElectricHeater>(&*building.hvacs[0]).unwrap();        
        assert_eq!(heater_name, h.name);        
        assert_eq!(*h.index().unwrap(), 0);
    }

    use crate::boundary::Boundary;
    #[test]
    fn write_io_doc(){
        let mut summary = "# Summary\n\n".to_string();
        // Add manually written chapters
        summary.push_str("- [Chapter 1](./chapter_1.md)\n");

        // Add automatic documentation
        let dir = "./ioreference/src";
        Boundary::print_doc(&dir, &mut summary).unwrap();
        Construction::print_doc(&dir, &mut summary).unwrap();
        HVACKind::print_doc(&dir, &mut summary).unwrap();
        Luminaire::print_doc(&dir, &mut summary).unwrap();
        Material::print_doc(&dir, &mut summary).unwrap();
        Space::print_doc(&dir, &mut summary).unwrap();
        Substance::print_doc(&dir, &mut summary).unwrap();
        // assert!(false)

        let summary_file = format!("{}/SUMMARY.md", dir);
        std::fs::write(summary_file, summary.as_bytes()).unwrap();

    }

    
}
