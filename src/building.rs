use geometry3d::polygon3d::Polygon3D;

use crate::substance::{Substance, SubstanceProperties};
use crate::material::{Material, MaterialProperties};
use crate::boundary::Boundary;
use crate::construction::Construction;
use crate::object_trait::ObjectTrait;
use crate::surface::Surface;
use crate::space::Space;
//use crate::boundary::Boundary;

pub struct Building {

    /// The name of the building
    name: String,

    /// The position of the Building in its contaning array
    index: usize,

    // materiality
    substances: Vec<Substance>,
    materials: Vec<Material>,
    constructions: Vec<Construction>,

    // geometry
    surfaces: Vec<Surface>,
    spaces: Vec<Space>,

}

impl ObjectTrait for Building {

    fn name(&self)-> &String {
        &self.name
    }

    fn class_name(&self)->String{
        "Building".to_string()
    }

    fn index(&self)->usize{
        self.index
    }

    /// Checks whether the objects in the building are all full
    fn is_full(&self)-> Result<(),String> {

        if self.substances.len()==0 {
            return Err(format!("{} '{}' has no Substances", self.class_name(), self.name))
        }
        for s in self.substances.iter(){
            match s.is_full(){
                Ok(_)=>{},
                Err(_)=>return s.error_is_not_full()                
            }            
        }

        if self.substances.len()==0 {
            return Err(format!("{} '{}' has no Materials", self.class_name(), self.name))
        }
        for s in self.materials.iter(){
            match s.is_full(){
                Ok(_)=>{},
                Err(_)=>return s.error_is_not_full()                
            }            
        }

        if self.substances.len()==0 {
            return Err(format!("{} '{}' has no Constructions", self.class_name(), self.name))
        }
        for s in self.constructions.iter(){
            match s.is_full(){
                Ok(_)=>{},
                Err(_)=>return s.error_is_not_full()                
            }            
        }
        
        if self.substances.len()==0 {
            return Err(format!("{} '{}' has no Surface", self.class_name(), self.name))
        }
        for s in self.surfaces.iter(){
            match s.is_full(){
                Ok(_)=>{},
                Err(_)=>return s.error_is_not_full()                
            }            
        }

        if self.substances.len()==0 {
            return Err(format!("{} '{}' has no Spaces", self.class_name(), self.name))
        }
        for s in self.spaces.iter(){
            match s.is_full(){
                Ok(_)=>{},
                Err(_)=>return s.error_is_not_full()                
            }            
        }
        // All good
        Ok(())
    }
}

impl Building {

    /// Creates an empty building
    pub fn new(name:String) -> Self {
        Self{
            name: name,
            index: 0,

            substances: Vec::new(),
            materials: Vec::new(),
            constructions: Vec::new(),
            surfaces: Vec::new(),
            spaces: Vec::new(),
        }
    }

    fn error_out_of_bounds<T>(&self, element_name: &str, i: usize)->Result<T,String>{
        Err(format!("{} number {} does not exist in {} '{}'", element_name, i, self.class_name(), self.name))
    }
    
    pub fn get_substances(&self)->&Vec<Substance>{
        &self.substances
    }

    pub fn get_materials(&self)->&Vec<Material>{
        &self.materials
    }

    pub fn get_constructions(&self)->&Vec<Construction>{
        &self.constructions
    }

    pub fn get_surfaces(&self)->&Vec<Surface>{
        &self.surfaces
    }

    pub fn get_spaces(&self)->&Vec<Space>{
        &self.spaces
    }


    /* SUBSTANCE */

    /// Adds a new empty Substance to the model
    pub fn add_substance(&mut self, name: String)-> usize {
        let i = self.substances.len();

        self.substances.push(Substance::new(name,i));

        i
    }

    /// Retrieves a substance from the Substances array 
    /// in the Building
    pub fn get_substance(&self, index: usize)->Result<&Substance, String>{
        if index >= self.substances.len(){
            return self.error_out_of_bounds("Substance", index)            
        }
        
        Ok(&self.substances[index])
    }

    /// Sets the properties to the substance located in a certain index
    /// of the Substances array in the Building object
    pub fn set_substance_properties(&mut self, index: usize, properties: SubstanceProperties)->Result<(),String>{
        if index >= self.substances.len(){
            return self.error_out_of_bounds("Substance", index)            
        }

        self.substances[index].set_properties(properties);
        Ok(())
    }

    


    /* MATERIAL */

    /// Adds a new empty Material to the model
    pub fn add_material(&mut self, name: String)-> usize {
        let i = self.materials.len();
        self.materials.push(Material::new(name,i));
        i
    }

    /// Retrieves a material from the Materials array 
    /// in the Building
    pub fn get_material(&self, index: usize)->Result<&Material, String>{
        if index >= self.materials.len(){
            return self.error_out_of_bounds("Material", index)            
        }
        
        Ok(&self.materials[index])
    }

    
    /// Sets a material surface
    pub fn set_material_substance(&mut self, material_index: usize, substance_index: usize)->Result<(),String>{
        if material_index >= self.materials.len(){
            return self.error_out_of_bounds("Material", material_index)
        }

        if substance_index >= self.substances.len(){
            return self.error_out_of_bounds("Substance", substance_index)
        }

        self.materials[material_index].set_substance(substance_index);
        Ok(())
    }

    /// Sets a material property
    pub fn set_material_properties(&mut self, material_index: usize, properties: MaterialProperties)->Result<(),String>{
        if material_index >= self.materials.len(){
            return self.error_out_of_bounds("Material", material_index)            
        }

        self.materials[material_index].set_properties(properties);
        Ok(())
    }


    /* CONSTRUCTION */

    /// Creates a new construction
    pub fn add_construction(&mut self, name: String)-> usize {
        let i = self.constructions.len();
        self.constructions.push(Construction::new(name,i));
        i
    }

    /// Retrieves a construction
    pub fn get_construction(&self, index: usize)->Result<&Construction, String>{
        if index >= self.constructions.len(){
            return self.error_out_of_bounds("Construction", index)            
        }
        
        Ok(&self.constructions[index])
    }

    /// Pushes a new Material layer to a construction 
    /// in the Building object
    pub fn add_material_to_construction(&mut self, construction_index: usize, material_index: usize)->Result<(),String>{

        if material_index >= self.materials.len(){
            return self.error_out_of_bounds("Material", material_index)
        }

        if construction_index >= self.constructions.len(){
            return self.error_out_of_bounds("Construction", construction_index)
        }

        self.constructions[construction_index].push_layer(material_index);

        Ok(())
    }

    /* SURFACE */

    /// Creates a new construction
    pub fn add_surface(&mut self, name: String) -> usize {
        let i = self.surfaces.len();
        self.surfaces.push(Surface::new(name,i));
        i
    }

    /// Retrieves a construction
    pub fn get_surface(&self, index: usize)->Result<&Surface, String>{
        if index >= self.surfaces.len(){
            return self.error_out_of_bounds("Surface", index)            
        }
        
        Ok(&self.surfaces[index])
    }

    /// Sets the front boundary of a Surface
    pub fn set_surface_front_boundary(&mut self, surface_index: usize, boundary: Boundary)->Result<(),String>{
        if surface_index >= self.surfaces.len(){
            return self.error_out_of_bounds("Surface", surface_index)
        }
        match boundary {
            Boundary::Ground | Boundary:: None => self.surfaces[surface_index].set_front_boundary(boundary),
            Boundary::Space(s) => {
                if s >= self.spaces.len(){
                    return self.error_out_of_bounds("Space", s)
                }else{
                    self.spaces[s].push_surface(surface_index);                    
                    self.surfaces[surface_index].set_front_boundary(boundary)
                }
            }
        }

        
    }

    /// Sets the back boundary of a Surface
    pub fn set_surface_back_boundary(&mut self, surface_index: usize, boundary: Boundary)->Result<(),String>{
        if surface_index >= self.surfaces.len(){
            return self.error_out_of_bounds("Surface", surface_index)
        }

        match boundary {
            Boundary::Ground | Boundary:: None => self.surfaces[surface_index].set_back_boundary(boundary),
            Boundary::Space(s) => {
                if s >= self.spaces.len(){
                    return self.error_out_of_bounds("Space", s)
                }else{
                    self.spaces[s].push_surface(surface_index);
                    self.surfaces[surface_index].set_back_boundary(boundary)
                }
            }
        }
    }

    /// Sets the polygon for a Surface
    pub fn set_surface_polygon(&mut self, surface_index: usize, p: Polygon3D)->Result<(),String>{
        if surface_index >= self.surfaces.len(){
            return self.error_out_of_bounds("Surface", surface_index)
        }

        self.surfaces[surface_index].set_polygon(p);

        Ok(())
    }

    /// Sets the construction of a surface
    pub fn set_surface_construction(&mut self, surface_index: usize, construction_index: usize)->Result<(),String>{
        if surface_index >= self.surfaces.len(){
            return self.error_out_of_bounds("Surface", surface_index)
        }

        if construction_index >= self.constructions.len(){
            return self.error_out_of_bounds("Construction", construction_index)
        }
        
        self.surfaces[surface_index].set_construction(construction_index);

        Ok(())
    }


    /* SPACES */

    /// Creates a new construction
    pub fn add_space(&mut self, name: String) -> usize {
        let i = self.spaces.len();
        self.spaces.push(Space::new(name,i));
        i
    }

    /// Retrieves a construction
    pub fn get_space(&self, index: usize)->Result<&Space, String>{
        if index >= self.spaces.len(){
            return self.error_out_of_bounds("Space", index)            
        }        
        Ok(&self.spaces[index])
    }

    /// Sets a space volume
    pub fn set_space_volume(&mut self, index: usize, volume: f64)->Result<(),String>{
        if index >= self.spaces.len(){
            return self.error_out_of_bounds("Space", index)            
        }        
        self.spaces[index].set_volume(volume);
        Ok(())
    }
    

}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing{
    use super::*;


    use crate::substance::{SubstanceProperties};
    
    #[test]
    fn substance(){

        let mut building = Building::new("Test Building".to_string());

        let subs_name = "Substance 0".to_string();
        let s0 = building.add_substance(subs_name.clone());
        {
            let s = building.get_substance(s0).unwrap();
            assert_eq!(&subs_name, s.name());
            assert_eq!(0, s.index());
            assert!(s.is_full().is_err());
        }
            
        let lambda = 1.23123;
        let rho = 1.2312312555;
        let c = 9.123128;
        building.set_substance_properties(s0, SubstanceProperties{
            thermal_conductivity: lambda,
            specific_heat_capacity: c,
            density: rho,
        }).unwrap();

        {
            let s = building.get_substance(s0).unwrap();
            assert!(s.is_full().is_ok());
            assert_eq!(s.thermal_diffusivity().unwrap(),lambda / rho/c);
            assert_eq!(s.density().unwrap(),rho);
            assert_eq!(s.specific_heat_capacity().unwrap(),c);        
        }
        

        // Add another one
        let subs_name = "Substance 1".to_string();
        let s1 = building.add_substance(subs_name.clone());
        {
            let s = building.get_substance(s1).unwrap();
            assert_eq!(&subs_name, s.name());
            assert_eq!(1, s.index());
            assert!(s.is_full().is_err())
        }

        assert!(building.get_substance(0).is_ok());
        assert!(building.get_substance(1).is_ok());
        assert!(building.get_substance(2).is_err());
    }



    use crate::material::{MaterialProperties};
    #[test]
    fn material(){

        let mut building = Building::new("The Building".to_string());

        // Add an empty material
        let mat_name = "The Material".to_string();
        let m0 = building.add_material(mat_name.clone());
        {
            let m = building.get_material(m0).unwrap();
            assert!(m.is_full().is_err());
        }

        let thickness = 3.21;
        building.set_material_properties(m0, MaterialProperties{
            thickness: thickness,
        }).unwrap();
        
        {
            let mat = building.get_material(m0).unwrap();
            assert!(mat.is_full().is_err());
            assert_eq!(mat.index(),0);
        }
        
        // Add a couple of substances
        building.add_substance("Substance_0".to_string());
        let subs_name = "Substance_1".to_string();
        let s1 = building.add_substance(subs_name.clone());
        assert_eq!(1,s1);
        {
            let s = building.get_substance(s1).unwrap();
            assert_eq!(s.index(),s1);

        }

        // these should fail... index out of bounds
        assert!(building.set_material_substance(m0, 34).is_err());
        assert!(building.set_material_substance(131, s1).is_err());
        assert!(building.set_material_substance(131, 34).is_err());
        // this should work
        assert!(building.set_material_substance(m0, s1).is_ok());

        {
            let mat = building.get_material(m0).unwrap();
            assert_eq!(mat.index(),m0);
            assert_eq!(mat.thickness().unwrap(),thickness);

        }

        assert!(building.get_material(0).is_ok());
        assert!(building.get_material(1).is_err());
        assert!(building.get_material(2).is_err());
        
    }


    #[test]
    fn construction(){

        let mut building = Building::new("The Building".to_string());

        // Add an empty material
        let mat_name = "The Material".to_string();
        let m0 = building.add_material(mat_name.clone());
        {
            let m = building.get_material(m0).unwrap();
            assert!(m.is_full().is_err());
        }

        let c_name = "The construction".to_string();
        let c0 = building.add_construction(c_name);

        assert!(building.add_material_to_construction(123, m0).is_err());
        assert!(building.add_material_to_construction(c0, 123).is_err());
        assert!(building.add_material_to_construction(123, 123).is_err());
        
        // This should work
        assert!(building.add_material_to_construction(c0, m0).is_ok());        
        {
            let c = building.get_construction(c0).unwrap();
            assert_eq!(1,c.n_layers());
        }
        assert!(building.add_material_to_construction(c0, m0).is_ok());
        {
            let c = building.get_construction(c0).unwrap();
            assert_eq!(2,c.n_layers());
        }
    
    }

    #[test]
    fn surface_space (){
        let mut building = Building::new("Test Building".to_string());

        let space_name = "Space 0".to_string();
        let space_index = building.add_space(space_name.clone());
        {
            let s = building.get_space(space_index).unwrap();
            assert_eq!(&space_name, s.name());
            assert_eq!(s.get_surfaces().len(),0);
            assert_eq!(0, s.index());
            assert!(s.is_full().is_err());
        }


        let s_name = "Surface 0".to_string();
        let s0 = building.add_surface(s_name.clone());
        {
            let s = building.get_surface(s0).unwrap();
            assert_eq!(&s_name, s.name());
            assert_eq!(0, s.index());
            assert!(s.is_full().is_err());
        }
        
        building.set_surface_front_boundary(s0, Boundary::Space(space_index)).unwrap();
        building.set_surface_back_boundary(s0, Boundary::Ground).unwrap();
        {
            let space = building.get_space(space_index).unwrap();
            let space_surfaces = space.get_surfaces();
            assert_eq!(space_surfaces.len(),1);
            assert_eq!(space_surfaces[0],s0);
            
            let s = building.get_surface(s0).unwrap();
            if let Boundary::Space(i) = s.front_boundary(){
                assert_eq!(*i,space_index);
            }else{
                assert!(false);
            }
            
            if let Boundary::Ground = s.back_boundary(){
                assert!(true);
            }else{
                assert!(false);
            }

            assert_eq!(0, s.index());
            assert!(s.is_full().is_err());
        }

        let s_name = "Surface 1".to_string();
        let _s1 = building.add_surface(s_name.clone());

        let s_name = "Surface 2".to_string();
        let s2 = building.add_surface(s_name.clone());
        {
            let s = building.get_surface(s2).unwrap();
            assert_eq!(&s_name, s.name());
            assert_eq!(2, s.index());
            assert!(s.is_full().is_err());
        }
        
        building.set_surface_front_boundary(s2, Boundary::Ground).unwrap();
        building.set_surface_back_boundary(s2, Boundary::Space(space_index)).unwrap();
        {
            let space = building.get_space(space_index).unwrap();
            let space_surfaces = space.get_surfaces();
            assert_eq!(space_surfaces.len(),2);
            assert_eq!(space_surfaces[0],s0);
            assert_eq!(space_surfaces[1],s2);

            let s = building.get_surface(s2).unwrap();
            if let Boundary::Space(i) = s.back_boundary(){
                assert_eq!(*i,space_index);
            }else{
                assert!(false);
            }
            
            if let Boundary::Ground = s.front_boundary(){
                assert!(true);
            }else{
                assert!(false);
            }

            assert_eq!(2, s.index());
            assert!(s.is_full().is_err());
        }


    }

    

    
}