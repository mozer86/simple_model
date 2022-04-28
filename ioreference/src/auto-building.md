# Building

  This object is utilized to group `Space` objects together for
  metering and/or shared values. For example, the number of storeys
  and the `ShelterClass` will help defining the `Infiltrations`


```rs
Building {
   name : string,
   n_storeys : int, // optional,
   shelter_class : ShelterClass, // optional,
   stack_coefficient : number, // optional,
   wind_coefficient : number, // optional,
}
```



#### name

  The name of the Building




#### n_storeys (*optional*)

  The number of storeys of this building.
 
  This value use used by the `AirFlow` module when a `Space` associated
  to this `Building` has been assigned an `EffectiveAirLeakageArea`
  infiltration. This value is required for calculating the Stack
  Coefficient ($C_s$) and the Wind Coefficient ($C_w$) of the
  `EffectiveAirLeakageArea` infiltration. $C_s$ and $C_w$ can be inputed
  directly by assigning values to the `stack_coefficient` and
  `wind_coefficient` fields, in which case the `n_storeys` field will
  be ignored.




#### shelter_class (*optional*)

  The `ShelterClass` of this building.
 
  This value use used by the `AirFlow` module when a `Space` associated
  to this `Building` has been assigned an `EffectiveAirLeakageArea`
  infiltration. This value is required for calculating the Wind
  Coefficient ($C_s$) of the
  `EffectiveAirLeakageArea` infiltration.  $C_w$ can be inputed
  directly by assigning values to the `wind_coefficient` field, in
  which case the `shelter_class` field will be ignored.




#### stack_coefficient (*optional*)

  The stack coefficient of this building, used for

 calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
  infiltration option.
 
  If not given, the number of storeys will be used for getting
  this values (based on EnergyPlus\' Engineering Reference).
 

 > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
  > of 3 storeys or less.




#### wind_coefficient (*optional*)

  The wind coefficient of this building, used for

 calculating infiltrations in `Spaces` that utilize the `EffectiveAirLeakageArea`
  infiltration option.
 
  If not given, the number of storeys will be used for getting
  this values (based on EnergyPlus\' Engineering Reference).
 

 > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
  > of 3 storeys or less.




