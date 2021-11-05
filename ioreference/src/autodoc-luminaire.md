# Luminaire

  A Luminaire


```rs
Luminaire {
	name : string
	max_power : number   // Optional
	target_space : Space   // Optional
}
```

## Fields



#### name

  The name of the Luminaire




#### max_power  (*optional*)

  The maximum power consumption




#### target_space  (*optional*)

  The space in which the space is located
 
  While this value is might not be relevant for
  e.g., lighting calculations, this is necessary for
  thermal simulations, in which the heat disipated by
  a luminaire will be disipated into the air of a thermal
  zone. So, if this is an exterior luminaire or if no thermal
  calculation is performed, this can be left empty.








## API

The following functions are available for simulating control algorithms
### Standard use

#### `luminaire_power_consumption(i : int)`
 Gets the `power_consumption` of the Luminaire in index `i`
#### `luminaire_power_consumption(name : string)`
 Gets the `power_consumption` of the Luminaire called `name`

#### `set_luminaire_power_consumption(i : int, v: number)`
 Sets the `power_consumption` of the Luminaire in index `i` to a value of `v`
#### `set_luminaire_power_consumption(name : string, v: number)`
 Sets the `power_consumption` of the Luminaire called `name` to a value of `v`

### Research mode only
