# Luminaire

  A Luminaire
  
  Please fill this doc


```rs
Luminaire {
	name : string
	max_power : number   // Optional
	target_space : Space   // Optional
}
```

## Fields



### name

  The name of the Luminaire




### max_power  (*optional*)

  The maximum power consumption




### target_space  (*optional*)

  The space in which the space is located
 
  While this value is might not be relevant for
  e.g., lighting calculations, this is necessary for
  thermal simulations, in which the heat disipated by
  a luminaire will be disipated into the air of a thermal
  zone. So, if this is an exterior luminaire or if no thermal
  calculation is performed, this can be left empty.




