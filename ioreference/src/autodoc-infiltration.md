# Infiltration

  An infiltration rate for a `Space`


## Constant

  A contant infiltration, specified in `m3/s`


```
Constant(number)
```

## Blast

  Sets the infiltration to the `DesignFlowRate` values using the 

 default from BLAST as described in the EnergyPlus\' Input Output reference


```
Blast(number)
```

## Doe2

  Sets the infiltration to the `DesignFlowRate` values using the 

 default from DOE-2 as described in the EnergyPlus\' Input Output reference


```
Doe2(number)
```

## DesignFlowRate

  Sets the infiltration to the `DesignFlowRate` values using an 
  arbitrary set of values. This option is based on EnergyPlus\'
  object of the same name
  
  

 The flow $\phi$ (in $m^3/s$) is calculated from the parameters $A$, $B$, $C$, $D$ and 
  $\phi_{design}$ as follows:
  

 $$ \phi = \phi_{design} (A + B|T_{space} - T_{outside}| + C\times W_{speed} + D\times W^2_{speed})$$
  
  The inputs to this object are $A$, $B$, $C$, $D$, $\phi_{design}$ .


```
DesignFlowRate(number,number,number,number,number)
```

## DetailedEffectiveAirLeakageArea

  Sets the infiltration based on `EffectiveLeakageArea` as 
  described in the EnergyPlus\' Input Output reference.
      
  
  $$ \phi = \frac{A_L}{1000} \sqrt{C_s \Delta T + C_w W^2_{speed}}$$
  
  where:
  * $A_L$ is the effecctive air leakage in $cm^2$ @ 4Pa
  * $C_s$ is the coefficient for stack induced infiltration
  * $C_w$ is the coefficient for wind induced infiltration
  
  The inputs to this object are $A_L$, $C_s$ and $C_w$


```
DetailedEffectiveAirLeakageArea(number,number,number)
```

## EffectiveAirLeakageArea

  The same as the `DetailedEffectiveAirLeakageArea` object, but 
  derives the $C_s$ and $C_w$ factors from the `Building` object
  associated with the `Space` that owns this infiltration.
  
  For this to work, the associated `Building` needs to have  been 
  assigned the properties `storeys` and a `shelter_class`.
  
  The parameter required is $A_L$; i.e., the effecctive air leakage 
  in $cm^2$ @ 4Pa    
  

 > **Note:** The `EffectiveAirLeakageArea` object is appropriate for buildings
  > of 3 storeys or less. 


```
EffectiveAirLeakageArea(number)
```

