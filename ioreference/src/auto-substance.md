# Substance

  A physical substance with physical—i.e., optical, thermal—properties.
 
  Note that, contrary to EnergyPlus\' `Materials`, `Substances` do not
  contain information about the thickness, which in Simple is given when

 creating a `Material`. The idea is to enable multiple materials of different
  thicknesses to reference the same material.
 

 > Note: Glazing substances are `Normal` substances with `solar_transmitance`

 and `visible_transmittance`. However, contrary to all other properties, this property
  does depend on the thickness of the substance. So, in order
  to build a coherent Glazing, you\'ll need to match this Substance
  with an appropriate Material


* **Normal**:   A normal (i.e., solid, homogeneous) substance such as glass,
  timber or concrete.    

* **Gas**:   A gas

