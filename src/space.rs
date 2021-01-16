use crate::heating_cooling::HeaterCooler;
use crate::luminaire::Luminaire;
use crate::object_trait::ObjectTrait;
use calendar::date::Date;

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
    surfaces: Vec<usize>,
    
    /// The indices of the surrounding Fenestration in the
    /// Building's Surfaces array 
    fenestrations: Vec<usize>,

    /// The Heating/Cooling devices in the space
    heating_cooling: Option<HeaterCooler>,

    /// The luminaire in the space
    luminaire: Option<Luminaire>,
}

impl ObjectTrait for Space {

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
            surfaces: Vec::new(),            
            fenestrations: Vec::new(),            
            heating_cooling: None,
            luminaire: None,
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

    /// Adds a fenestration reference to the Space's 
    /// surfaces array
    pub fn push_fenestration(&mut self, s_index: usize) {                
        self.fenestrations.push(s_index)
    }

    /// retrieves the fenestrations
    pub fn get_fenestrations(&self)->&Vec<usize>{
        &self.fenestrations
    }

    pub fn get_importance(&self, _time: Date)->f64{
        return 1.0
    }
    
    /* ********** */
    /* LUMINAIRES */
    /* ********** */

    /// Adds a Luminaire to the Space. Returns an error if there
    /// was a Luminaire already there.
    pub fn add_luminaire(&mut self, luminaire: Luminaire)->Result<(),String>{
        if self.luminaire.is_some(){
            return Err(format!("Trying to replace {} of {} '{}'", luminaire.class_name(), self.class_name(), self.name))
        }
        self.luminaire = Some(luminaire);
        Ok(())
    }

    /// Sets the power for the luminaires in the Space.
    /// Returns an error if there are no luminaires in the Space.
    pub fn set_luminaire_max_power(&mut self, power: f64)->Result<(),String>{
        match &mut self.luminaire{
            Some(h) => {
                h.set_max_power(power);
                Ok(())
            },
            None => Err(format!("There are no Luminaires in {} '{}'", self.class_name(), self.name()))
        }
    }
    
    /// Retrieves the Luminaire
    pub fn get_luminaire(&self)->Option<&Luminaire>{
        match &self.luminaire{
            Some(v)=>Some(v),
            None => None
        }
    }

    /// Retrieves the state index of the Luminaires in the Space,
    /// if any
    pub fn get_luminaires_state_index(&self)->Option<usize>{
        match &self.luminaire{
            Some(h) => Some(h.state_index()),
            None => None
        }
    }

    /* *************** */
    /* HEATING/COOLING */
    /* *************** */

    /// Retrieves the HeatingCooling
    pub fn get_heating_cooling(&self)->Option<&HeaterCooler>{
        match &self.heating_cooling{
            Some(v)=>Some(v),
            None => None
        }
    }

    /// Retrieves the state index of the heating/cooling system
    pub fn get_heating_cooling_state_index(&self)->Option<usize>{
        match &self.heating_cooling{
            Some(h) => Some(h.state_index()),
            None => None
        }
    }

    
    /// Adds a Heating/Cooling of the space. Returns an error if there was
    /// a system already there.
    pub fn add_heating_cooling(&mut self, system: HeaterCooler)->Result<(),String>{
        if self.heating_cooling.is_some(){
            return Err(format!("Trying to replace {} of {} '{}'", system.class_name(), self.class_name(), self.name))
        }
        self.heating_cooling = Some(system);
        Ok(())
    }

    /// Sets the maximum heating power.
    pub fn set_max_heating_power(&mut self, power: f64)->Result<(),String>{
        match &mut self.heating_cooling{
            Some(v)=>{
                v.set_max_heating_power(power);
                Ok(())
            },
            None => Err(format!("{} '{}' has no Heating/Cooling system... cannot set maximum heating power", self.class_name(), self.name()))
        }
    }

    /// Sets the maximum cooling power.
    pub fn set_max_cooling_power(&mut self, power: f64)->Result<(),String>{
        match &mut self.heating_cooling{
            Some(v)=>{
                v.set_max_cooling_power(power);
                Ok(())
            },
            None => Err(format!("{} '{}' has no heating/cooling system... cannot set maximum cooling power", self.class_name(), self.name()))
        }
    }

    
}