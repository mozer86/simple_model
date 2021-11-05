# Substance

  Represents a Substance; that is to say, a physical
  materiality with physical properties. The name Substance
  has been chosen instead of Material to respect EnergyPlus\'
  and other software\'s terminology (which does not include
  Substace, but it does include Material, which is essentially
  a Substance with a thickness).


```rs
Substance {
	name : string
	thermal_conductivity : number   // Optional
	specific_heat_capacity : number   // Optional
	density : number   // Optional
}
```

## Fields



#### name

  The name of the Substance. Should be unique for each
  Material in the SimpleModel object    




#### thermal_conductivity  (*optional*)

  The thermal conductivity of the substance in W/m.K




#### specific_heat_capacity  (*optional*)

  The specific heat capacity of the substance in J/kg.K




#### density  (*optional*)

  The density of the substance in kg/m3




