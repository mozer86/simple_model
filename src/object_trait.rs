pub trait ObjectTrait {

    /// Returns a reference to the name of the 
    /// Substance
    fn name(&self)->&String;

    /// Classname
    fn class_name(&self)->&str;

    /// Returns the index of the object in its 
    /// containing array
    fn index(&self)->usize;

    /// Checks whether the object contains
    /// all the information required
    fn is_full(&self)->bool;

    /// Returns an error stating that the object is not 
    /// valid
    fn error_is_not_full<T>(&self)->Result<T,String>{
        Err(format!("{} '{}' is empty", self.class_name(), self.name()))
    }

    /// Returns an error informing that an empty object
    /// is being used, unsuccesfuly
    fn error_using_empty<T>(&self)->Result<T,String>{
        Err(format!("Attempting to use {} '{}', which is empty", self.class_name(), self.name()))
    }

}