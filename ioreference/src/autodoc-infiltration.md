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


```
DesignFlowRate(number,number,number,number,number)
```

## EffectiveAirLeakageArea

  Sets the infiltration based on `EffectiveLeakageArea` as 
  described in the EnergyPlus\' Input Output reference.
  
  The area is in cm2 estimated for a pressure differencial 
  of 4Pa


```
EffectiveAirLeakageArea(number)
```

