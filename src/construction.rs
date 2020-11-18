use crate::object_trait::ObjectTrait;

/// An object representing a multilayer 
/// Construction; that is to say, an array of 
/// Materials
pub struct Construction{
        
    /// The name of the Construction object. 
    /// Must be unique within the model
    name: String,

    /// The index of the Construction object within
    /// the constructions property in the Building object
    index: usize,

    /// The indices of the Material objects in the 
    /// materials property of the Building object
    layers: Vec< usize >,
    
    
}

impl ObjectTrait for Construction{

    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->&str{
        "Construction"
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->bool{
        self.layers.len() != 0
    }
}

    
impl Construction {

    /*
    /// Create a new placeholder construction (e.g. empty and without )
    pub fn new(name: String, index: usize)-> Self {
        Construction {            
            name: name,            
            index: index,            
            layers: Vec::new()
        }
    }
    */

    

    /// Returns the number of layers in the object
    pub fn n_layers(&self)->usize{
        self.layers.len()
    }

    /// Returns the number of the 
    pub fn get_material_index(&self,i:usize)->Result<usize, String>{
        if self.layers.len() == 0 {
            return self.error_using_empty();
        }

        match self.layers.get(i){
            Some(v) => Ok(*v),
            None => {
                return Err(format!("Index out of bounds... trying to access layer {} of {} '{}', but it has only {} layers", i, self.class_name(), self.name, self.layers.len()));
            }
        }
    }

    
    
}



/***********/
/* TESTING */
/***********/



#[cfg(test)]
mod testing{
    

}