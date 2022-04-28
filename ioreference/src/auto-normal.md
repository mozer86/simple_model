# Normal

  Represents an opaque physical material
  with common physical properties. (e.g.,
  timber, concrete, brick)


```rs
Normal {
   name : string,
   thermal_conductivity : number, // optional,
   specific_heat_capacity : number, // optional,
   density : number, // optional,
   solar_absorbtance : number, // optional,
   thermal_absorbtance : number, // optional,
   solar_transmittance : number, // optional,
   visible_transmittance : number, // optional,
}
```



#### name

  The name of the Substance. Should be unique for each
  Substance in the SimpleModel object    




#### thermal_conductivity (*optional*)

  The thermal conductivity of the substance in W/m.K




#### specific_heat_capacity (*optional*)

  The specific heat capacity of the substance in J/kg.K




#### density (*optional*)

  The density of the substance in kg/m3




#### solar_absorbtance (*optional*)

  Solar absorbtance (from 0 to 1)




#### thermal_absorbtance (*optional*)

  Front thermal absorbtance (i.e., emissitivy; from 0 to 1)




#### solar_transmittance (*optional*)

  The solar transmittance at normal incidence (from 0 to 1)
 
  Please note that, contrary to all other properties, this property
  does depend on the thickness of the substance. So, in order
  to build a coherent Glazing, you\'ll need to match this Substance
  with an appropriate Material




#### visible_transmittance (*optional*)

  The visible transmittance at normal incidence (from 0 to 1)
 
  Please note that, contrary to all other properties, this property
  does depend on the thickness of the substance. So, in order
  to build a coherent Glazing, you\'ll need to match this Substance
  with an appropriate Material




