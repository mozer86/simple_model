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
use derive::SimpleInputOutput;
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


#[derive(Default, SimpleInputOutput)]
pub struct SimpleModel {
    /// The name of the building
    pub name: String,

    // materiality
    pub substances: Vec<Substance>,
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
        
        let bytes = match fs::read(filename.clone()){
            Ok(v)=>v,
            Err(_)=>{
                return Err(format!("Could not read SIMPLE file '{}'", filename))
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
    
    
    use crate::hvac;
    use crate::substance;
    use crate::boundary::Boundary;
    use crate::building::{Building, /*ShelterClass*/};
    // use crate::infiltration::Infiltration;
    #[test]
    fn write_io_doc(){
        let mut summary = "# Input/Output reference guide\n\n".to_string();
        
        // Add automatic documentation
        // let dir = "../src";
        let dir = "./book/src";
        let summary_file = format!("{}/SUMMARY.md", dir);
        
        if !std::path::Path::new(&summary_file).exists(){            
            return;
        }

        // clear summary
        let f = std::fs::File::create(&summary_file).unwrap();
        f.set_len(0).unwrap();
        

        // Boundary
        Boundary::print_doc(&dir, &mut summary).unwrap();
        
        // Building
        Building::print_doc(&dir, &mut summary).unwrap();
        
        // Construction
        Construction::print_doc(&dir, &mut summary).unwrap();
        
        // Fenestration
        Fenestration::print_doc(&dir, &mut summary).unwrap();
        Fenestration::print_api_doc(&dir, &mut summary).unwrap();
        
        // HVAC
        HVAC::print_doc(&dir, &mut summary).unwrap();
        
        summary.push_str(&format!("\t"));
        hvac::ElectricHeater::print_doc(&dir, &mut summary).unwrap();
        hvac::ElectricHeater::print_api_doc(&dir, &mut summary).unwrap();
        
        summary.push_str(&format!("\t"));
        hvac::IdealHeaterCooler::print_doc(&dir, &mut summary).unwrap();
        hvac::IdealHeaterCooler::print_api_doc(&dir, &mut summary).unwrap();

        // Infiltration
        // Infiltration::print_doc(&dir, &mut summary).unwrap();
        
        // Luminaire
        Luminaire::print_doc(&dir, &mut summary).unwrap();
        Luminaire::print_api_doc(&dir, &mut summary).unwrap();

        // Material
        Material::print_doc(&dir, &mut summary).unwrap();
        
        // Space
        Space::print_doc(&dir, &mut summary).unwrap();
        Space::print_api_doc(&dir, &mut summary).unwrap();
        
        // Substance
        Substance::print_doc(&dir, &mut summary).unwrap();
                
        summary.push_str(&format!("\t"));
        substance::Normal::print_doc(&dir, &mut summary).unwrap();
        // substance::Normal::print_api_doc(&dir, &mut summary).unwrap();
        
        


        
        // ShelterClass::print_doc(&dir, &mut summary).unwrap();
        
        Surface::print_doc(&dir, &mut summary).unwrap();
        Surface::print_api_doc(&dir, &mut summary).unwrap();
        

        
        let current_summary = fs::read_to_string(summary_file.clone()).expect("Could not read summary file");
        let whole_summary = format!("{}\n\n{}", current_summary, summary);
        std::fs::write(summary_file, whole_summary.as_bytes()).unwrap();

    }

    #[test]
    fn test_read_file(){
        let (model, _state_header) = SimpleModel::from_file("./test_data/box.spl".to_string()).unwrap();
        
        assert_eq!(model.substances.len(), 1);
        assert_eq!(model.substances[0].name().clone(), "the substance");
        
        assert_eq!(model.materials.len(), 2);
        assert_eq!(model.materials[0].name, "the material");        
        assert_eq!(model.materials[1].name, "another material");
        
        assert_eq!(model.constructions.len(), 1);
        assert_eq!(model.constructions[0].name, "the construction");        
        assert!(Rc::ptr_eq(&model.constructions[0].materials[0], &model.materials[0]));

        assert_eq!(model.surfaces.len(), 1);
        assert_eq!(model.surfaces[0].name, "the surface");
        assert!(Rc::ptr_eq(&model.surfaces[0].construction, &model.constructions[0]));

        assert_eq!(model.spaces.len(), 1);
        assert_eq!(model.spaces[0].name, "Bedroom");        


        assert_eq!(model.buildings.len(), 1);
        assert_eq!(model.buildings[0].name, "Main campus");        


        assert_eq!(model.hvacs.len(), 1);
        if let HVAC::ElectricHeater(heater) = &model.hvacs[0] {
            assert_eq!("Bedrooms heater", heater.name)
        }else{
            panic!("Was not an electric heater!");
        }
        


        
    }

    use crate::substance::Normal;

    #[test]
    fn building_substance() {
        let mut building = SimpleModel::new("Test Model".to_string());

        let subs_name = "Substance 0".to_string();
        let substance = Normal::new(subs_name.clone()).wrap();

        let s0 = building.add_substance(substance);

        let s = &building.substances[0];        
        assert_eq!(subs_name, s.name().clone());
        assert_eq!(subs_name, s0.name().clone());

        #[allow(irrefutable_let_patterns)]
        if let Substance::Normal(s) = &s0 {
            assert_eq!(*s.index().unwrap(), 0);
        }else{
            panic!("asd")
        }
        
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
        
        let space = Space::new("some space".to_string());
        let state_index = state_header.push(SimulationStateElement::SpaceInfiltrationVolume(0), 2.1);
        space.set_infiltration_volume_index(state_index);
        let state_index = state_header.push(SimulationStateElement::SpaceDryBulbTemperature(0), 22.2);
        space.set_dry_bulb_temperature_index(state_index);
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

            

            // Electric
            let electric = hvac(\"electric heater\");
            let power = electric.power_consumption;
            print(`Electric power consumption is ${power} W`);
            electric.power_consumption = 99.1;
            let power = electric.power_consumption;
            print(`Electric power consumption is ${power} W`);

            // Ideal
            let ideal = hvac(\"ideal hvac\");
            let power = ideal.power_consumption;
            print(`Ideal power consumption is ${power} W`);
            ideal.power_consumption = 912.1;
            let power = ideal.power_consumption;
            print(`Ideal power consumption is ${power} W`);

            print(\"BY INDEX NOW ---->\");

            

            // Electric
            let electric = hvac(0);
            let power = electric.power_consumption;
            print(`Electric power consumption is ${power} W`);
            
            // Ideal
            let ideal = hvac(1);
            let power = ideal.power_consumption;
            print(`Ideal power consumption is ${power} W`);
            

            // Temperature
            let the_space = space(\"some space\");
            let temp = the_space.dry_bulb_temperature;
            print(`Temp is ${temp}`)            
            
        ").unwrap();

        let _result : () = engine.eval_ast(&ast).unwrap();

        



    }
    
}
