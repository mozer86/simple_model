# Gas

  Represents a Normal; that is to say, a physical
  materiality with physical properties. The name Normal
  has been chosen instead of Material to respect EnergyPlus\'
  and other software\'s terminology (which does not include
  Substace, but it does include Material, which is essentially
  a Normal with a thickness).


```rs
Gas {
   name : string,
   kind : StandardGas, // optional,
}
```



#### name

  The name of the Normal. Should be unique for each
  Material in the SimpleModel object    




#### kind (*optional*)

  A predefined gas



