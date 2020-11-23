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

    fn class_name(&self)->String{
        "Construction".to_string()
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->Result<(),String>{
        if self.layers.len() != 0{
            Ok(())
        }else{
            self.error_is_not_full()
        }
    }
}

    
impl Construction {

    
    /// Create a new empty Construction ...
    /// The index does not have any meaning if the Construction is 
    /// self-contained; but it becomes meaningful when it is part of an
    /// Array. For instance, when inserting a new Construction to the     
    /// Building object, the latter chooses the appropriate index
    pub fn new(name: String, index: usize)-> Self {
        Construction {            
            name: name,            
            index: index,            
            layers: Vec::new()
        }
    }

    /// Borrows the Layers vector
    pub fn layers(&self)->&Vec<usize>{
        &self.layers
    }

    /// Returns the number of layers in the object
    pub fn n_layers(&self)->usize{
        self.layers.len()
    }

    /// Returns the number of the 
    pub fn get_layer_index(&self,i:usize)->Result<usize, String>{
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

    /// adds another layer to the Construction.
    pub fn push_layer(&mut self, layer_index: usize){
        self.layers.push(layer_index)
    }
    
    
}



/***********/
/* TESTING */
/***********/



#[cfg(test)]
mod testing{
    use super::*;

    #[test]
    fn test_basic(){
        let name = "The construction".to_string();
        let index = 12312;
        let mut c = Construction::new(name.clone(),index);
        assert_eq!(&name, c.name());
        assert_eq!(index, c.index());
        assert_eq!(0, c.n_layers());
        assert!(c.is_full().is_err());

        let layer0 = 23;
        c.push_layer(layer0);
        assert_eq!(1, c.n_layers());
        assert_eq!(layer0, c.get_layer_index(0).unwrap());
        assert!(c.get_layer_index(1).is_err());

        let layer1 = 412;
        c.push_layer(layer1);
        assert_eq!(2, c.n_layers());
        assert_eq!(layer1, c.get_layer_index(1).unwrap());
        assert!(c.get_layer_index(1).is_ok());
        assert!(c.get_layer_index(2).is_err());

    }

}