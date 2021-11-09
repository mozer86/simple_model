# Building

  This object is utilized to group `Space` objects together for 
  metering and/or shared values. For example, the number of storeys
  and the `ShelterClass` will help defining the `Infiltrations`


```rs
Building {
	name : string
	storeys : integer   // Optional
	stack_coefficient : number   // Optional
	wind_coefficient : number   // Optional
	shelter_class : ShelterClass   // Optional
}
```

## Fields



#### name

  The name of the Building




#### storeys  (*optional*)

  The number of storeys of this building




#### stack_coefficient  (*optional*)

  The stack coefficient of this building, used for 

 calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
  infiltration option.
  
  If not given, the number of storeys will be used for getting 
  this values (based on EnergyPlus\' Engineering Reference). 
 

 > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
  > of 3 storeys or less. 




#### wind_coefficient  (*optional*)

  The wind coefficient of this building, used for 

 calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
  infiltration option.
  
  If not given, the number of storeys will be used for getting 
  this values (based on EnergyPlus\' Engineering Reference). 
  

 > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
  > of 3 storeys or less. 




#### shelter_class  (*optional*)

  The `ShelterClass` of this building. 




