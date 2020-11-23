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

    fn class_name(&self)->String {
        "Space".to_string()
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->Result<(),String>{
        if self.volume.is_some() && self.surfaces.len() > 0 {
            Ok(())
        }else{
            self.error_is_not_full()
        }
    }
}

impl Space {

    /// Creates a new Space
    pub fn new(name: String, index: usize)->Self{
        Self{
            name: name,
            index: index,
            volume: None,
            surfaces: Vec::new()
        }
    }

    /// Returns the volume of the space
    pub fn volume(&self)->Result<f64,String>{
        match self.volume{
            Some(v)=>Ok(v),
            None => self.error_using_empty()
        }
    }

    /// Sets the volume of the space
    pub fn set_volume(&mut self, v: f64){
        self.volume = Some(v);
    }
    

    /// Adds a surface reference to the Space's 
    /// surfaces array
    pub fn push_surface(&mut self, s_index: usize) {                
        self.surfaces.push(s_index)
    }

    /// retrieves the surfaces
    pub fn get_surfaces(&self)->&Vec<usize>{
        &self.surfaces
    }

    
}