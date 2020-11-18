use crate::object_trait::ObjectTrait;

/// The representation of a physical layer-Material.
/// That is to say, a layer of a certain thickness 
/// made of a certain Substance
pub struct Material {   

    /// The name of the material object
    name: String,

    
    /// The position of this object in the materials vector
    /// owned by the Building object
    index: usize,
    
    /// The index of the substance of which
    /// the material is made of in vector referenced
    /// by its substances property
    substance: Option<usize>,

    /// The physical properties of the Material
    properties: Option<MaterialProperties>
}

pub struct MaterialProperties{

    /// The physical thickness of this material, in meters
    thickness: f64,
}

impl ObjectTrait for Material{
    
    fn name(&self)->&String{
        &self.name
    }

    fn class_name(&self)->&str{
        "Material"
    }

    fn index(&self)->usize{
        self.index
    }

    fn is_full(&self)->bool{
        self.properties.is_some() && self.substance.is_some()
    }
}

impl Material {

    /// Returns the thicnkess of this Material
    pub fn thickness(&self)->Result<f64,String>{
        match &self.properties{
            Some(p)=> Ok(p.thickness),
            None => self.error_using_empty()            
        }
    }

    /// Retrieves the substance index
    pub fn get_substance_index(&self)->Option<usize>{
        self.substance
    }    
    
}




/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing{
    use super::*;
    use crate::substance::{Substance,SubstanceProperties};


    #[test]
    fn test_new_ok(){
                
        let mut substances : Vec<Substance> = Vec::new();
        substances.push(Substance{            
            name: "A Substance".to_string(),
            index: substances.len(),
            properties: Some(SubstanceProperties{
                thermal_conductivity: 1.2,
                specific_heat_capacity: 1.0,
                density: 1.2,
            })
        });

        // Create the material
        let name = "The Material".to_string();
        let thickness = 1.123123;
        let index = 23;        
        let material = Material{
            name: name,
            index: index,
            substance: Some(0),
            properties: Some(MaterialProperties{
                thickness: thickness
            })
        };
        
        assert_eq!(material.name(),&"The Material".to_string());

        // Modify the substances vector        
        substances.push(Substance{            
            name: "A Substance".to_string(),
            index: substances.len(),
            properties:Some(SubstanceProperties{
                thermal_conductivity: 1.2*2.0,
                specific_heat_capacity: 1.0*2.0,
                density: 1.2*2.0,
            })
        });

        // Test material thickness
        assert_eq!(material.thickness(),thickness);

        }


}