use crate::object_trait::ObjectTrait;


/// Represents a space within a building. This will
/// often be a room, but it might also be half a room
pub struct Space {
    /// The name of the space
    name: String,

    /// The index of the Space in its containing array
    index: usize,

    /// Volume of the space
    volume: Option<f64>,    
    
    /// The indices of the surrounding Surfaces in the
    /// Building's Surfaces array 
    surfaces: Vec<usize>
}

impl ObjectTrait for Space{

    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->&str {
        "Space"
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->bool{
        self.volume.is_some() && self.surfaces.len() > 0
    }
}

impl Space {

    /// Returns the volume of the space
    pub fn volume(&self)->Result<f64,String>{
        match self.volume{
            Some(v)=>Ok(v),
            None => self.error_using_empty()
        }
    }
    

    /// Adds a surface reference to the Space's 
    /// surfaces array
    pub fn push_surface(&mut self, s_index: usize) {                
        self.surfaces.push(s_index)
    }
}