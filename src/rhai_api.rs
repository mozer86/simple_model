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

use std::rc::Rc;
use std::cell::RefCell;
use rhai::{Engine, EvalAltResult};
use crate::Float;
use crate::simulation_state::SimulationState;
use crate::model::SimpleModel;


fn as_usize(v: i64) -> Result<usize,Box<EvalAltResult>> {
    if v < 0 {
        return Err("Expecting a positive number".into());
    }
    Ok(v as usize)
}

/// Registers the functions used to operate the building
pub fn register_control_api(engine : &mut Engine, model: &Rc<SimpleModel>, state: &Rc<RefCell<SimulationState>>){


    let new_mod = Rc::clone(model);
    // let new_state = Rc::clone(state);
    engine.register_fn("count_spaces", move || {
        new_mod.spaces.len() as i32
    });

    let new_mod = Rc::clone(model);
    let new_state = Rc::clone(state);
    engine.register_result_fn("set_space_infiltration_volume", move |i: i64, v: Float| {
        let i = as_usize(i)?;
        if i >= new_mod.spaces.len(){
            return Err(format!("Trying to retreive space {}, but model only has {}", i, new_mod.spaces.len()).into());
        }
        let space = &new_mod.spaces[i];
        let state_ptr = &mut *new_state.borrow_mut();
        space.set_infiltration_volume(state_ptr, v);

        Ok(())
    });

}