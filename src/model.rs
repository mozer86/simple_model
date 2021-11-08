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
use crate::scanner::SimpleScanner;
use std::rc::Rc;
use std::fs;
use crate::hvac::*;
use building_state_macro::{SimpleObjectBehaviour};
use crate::SimulationStateHeader;

use crate::{
    Surface, 
    Space, 
    Building,
    Substance,
    Material,
    Construction,
    Luminaire,
    Fenestration
};


#[derive(Default, SimpleObjectBehaviour)]
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
    pub buildings : Vec<Rc<Building>>,

    /// The windows and doors in the surface    
    pub fenestrations: Vec<Rc<Fenestration>>,

    /// The Heating/Cooling devices in the space
    pub hvacs: Vec<HVAC>,

    /// Luminaires
    pub luminaires: Vec<Rc<Luminaire>>,
}


impl SimpleModel{

    pub fn from_file(filename: String)->Result<(Self, SimulationStateHeader), String>{
        
        let bytes = match fs::read(filename){
            Ok(v)=>v,
            Err(e)=>{
                return Err(format!("{}", e))
            }
        };
        let mut scanner = SimpleScanner::new(&bytes, 1);
        scanner.parse_model()
    }
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;
    use crate::hvac::{
        ElectricHeater,
        IdealHeaterCooler
    };
    

    use crate::boundary::Boundary;
    use crate::building::{Building, ShelterClass};
    use crate::infiltration::Infiltration;
    #[test]
    fn write_io_doc(){
        let mut summary = "# Summary\n\n".to_string();
        // Add manually written chapters
        summary.push_str("- [Chapter 1](./chapter_1.md)\n");

        // Add automatic documentation
        let dir = "./ioreference/src";
        Boundary::print_doc(&dir, &mut summary).unwrap();
        
        Building::print_doc(&dir, &mut summary).unwrap();
        
        Construction::print_doc(&dir, &mut summary).unwrap();
        
        Fenestration::print_doc(&dir, &mut summary).unwrap();
        Fenestration::print_api_doc(&dir, &mut summary).unwrap();
        
        /* HVAC GROUP */
        HVAC::print_doc(&dir, &mut summary).unwrap();
        
        summary.push_str(&format!("\t"));
        ElectricHeater::print_doc(&dir, &mut summary).unwrap();
        ElectricHeater::print_api_doc(&dir, &mut summary).unwrap();
        
        summary.push_str(&format!("\t"));
        IdealHeaterCooler::print_doc(&dir, &mut summary).unwrap();
        IdealHeaterCooler::print_api_doc(&dir, &mut summary).unwrap();

        Infiltration::print_doc(&dir, &mut summary).unwrap();
        
        Luminaire::print_doc(&dir, &mut summary).unwrap();
        Luminaire::print_api_doc(&dir, &mut summary).unwrap();

        Material::print_doc(&dir, &mut summary).unwrap();
        
        Space::print_doc(&dir, &mut summary).unwrap();
        Space::print_api_doc(&dir, &mut summary).unwrap();
        
        Substance::print_doc(&dir, &mut summary).unwrap();
        
        ShelterClass::print_doc(&dir, &mut summary).unwrap();
        
        Surface::print_doc(&dir, &mut summary).unwrap();
        Surface::print_api_doc(&dir, &mut summary).unwrap();
        // assert!(false)

        let summary_file = format!("{}/SUMMARY.md", dir);
        std::fs::write(summary_file, summary.as_bytes()).unwrap();

    }

    #[test]
    fn test_read_file(){
        let (model, _state_header) = SimpleModel::from_file("./test_data/box.spl".to_string()).unwrap();
        
        assert_eq!(model.substances.len(), 1);
        assert_eq!(model.substances[0].name, "the substance");
        
        assert_eq!(model.materials.len(), 2);
        assert_eq!(model.materials[0].name, "the material");        
        assert_eq!(model.materials[1].name, "another material");
        
        assert_eq!(model.constructions.len(), 1);
        assert_eq!(model.constructions[0].name, "the construction");        
        assert!(Rc::ptr_eq(&model.constructions[0].materials[0], &model.materials[0]));

        assert_eq!(model.surfaces.len(), 1);
        assert_eq!(model.surfaces[0].name, "the surface");
        assert!(Rc::ptr_eq(&model.surfaces[0].construction, &model.constructions[0]));

        
    }

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
        let mut state_header = SimulationStateHeader::new();
        let mut building = SimpleModel::new("Test Model".to_string());

        let heater_name = "Heater".to_string();
        let heater = ElectricHeater::new(heater_name.clone());

        let h0 = building.add_hvac(heater.wrap(), &mut state_header);

        if let HVAC::ElectricHeater(h) = h0{
            assert_eq!(heater_name, h.name);        
            assert_eq!(*h.index().unwrap(), 0);
        }

        if let HVAC::ElectricHeater(h) = &building.hvacs[0]{
            assert_eq!(heater_name, h.name);        
            assert_eq!(*h.index().unwrap(), 0);
        }
    }

    


    use crate::simulation_state::{SimulationStateHeader};
    
    use crate::simulation_state_element::SimulationStateElement;
    use std::cell::RefCell;
    use crate::rhai_api::*;    
    #[test]
    fn test_api(){

        let mut model = SimpleModel::new("The Model".to_string());
        let mut state_header = SimulationStateHeader::new();
        
        let electric = ElectricHeater::new("electric heater".to_string());
        let electric = model.add_hvac(electric.wrap(), &mut state_header);
        let ideal = IdealHeaterCooler::new("ideal hvac".to_string());
        let ideal = model.add_hvac(ideal.wrap(), &mut state_header);
        
        let state_index = state_header.push(SimulationStateElement::SpaceInfiltrationVolume(0), 2.1);
        let space = Space::new("some space".to_string());
        space.set_infiltration_volume_index(state_index);
        model.add_space(space);

        let mut state = state_header.take_values().unwrap();        
        
        if let HVAC::ElectricHeater(hvac) = electric{
            hvac.set_heating_cooling_consumption(&mut state, 91.2);
        }
        
        if let HVAC::IdealHeaterCooler(hvac) = ideal{
            hvac.set_heating_cooling_consumption(&mut state, 23.14);
        }
        
        // Wrap and send to the Heap
        let state = Rc::new(RefCell::new(state));
        let model = Rc::new(model);
        let mut engine = rhai::Engine::new();

        register_control_api(&mut engine, &model, &state, true);

        let ast = engine.compile("
            
            let some_space = space(\"some space\");
            let vol = some_space.infiltration_volume;
            print(`Infiltration volume is ${vol} `);
            some_space.infiltration_volume = 3.1415;
            let vol = some_space.infiltration_volume;
            print(`Infiltration volume is ${vol} `);

            let vol = space(0).infiltration_volume;
            print(`Infiltration volume is ${vol} `);

            print(\"NEXT ---->\");

            // some_space.dry_bulb_temperature = 22;

            // Ideal
            let ideal = hvac(\"ideal hvac\");
            let power = ideal.power_consumption;
            print(`Ideal power consumption is ${power} W`);
            ideal.power_consumption = 1;
            let power = ideal.power_consumption;
            print(`Ideal power consumption is ${power} W`);

            // Electric
            let electric = hvac(\"electric heater\");
            let power = electric.power_consumption;
            print(`Electric power consumption is ${power} W`);
            electric.power_consumption = 99.;
            let power = electric.power_consumption;
            print(`Electric power consumption is ${power} W`);
            
        ").unwrap();

        let _result : () = engine.eval_ast(&ast).unwrap();

        



    }
    
}
