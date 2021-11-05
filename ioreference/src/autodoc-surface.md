# Surface

  A fixed surface in the building (or surroundings). This can be of
  any Construction, transparent or not.


```rs
Surface {
	name : string
	vertices : [ number, ...]
	construction : Construction
	front_boundary : Boundary   // Optional
	back_boundary : Boundary   // Optional
}
```

## Fields



#### name

  The name of the surface




#### vertices

  An array of Numbers representing the vertices of the 
  surface. The length of this array must be divisible by 3.




#### construction

  The index of the construction in the SimpleModel\'s
  Construction array    




#### front_boundary  (*optional*)

  A reference to the Boundary in front of the Surface




#### back_boundary  (*optional*)

  A reference to the Boundary in back of the Surface








## API

The following functions are available for simulating control algorithms
### Standard use



#### `surface_front_temperature(i : int)`
 Gets the `front_temperature` of the Surface in index `i`
#### `surface_front_temperature(name : string)`
 Gets the `front_temperature` of the Surface called `name`
#### `surface_back_temperature(i : int)`
 Gets the `back_temperature` of the Surface in index `i`
#### `surface_back_temperature(name : string)`
 Gets the `back_temperature` of the Surface called `name`
### Research mode only

#### `set_surface_front_temperature(i : int, v: number)`
 Sets the `front_temperature` of the Surface in index `i` to a value of `v`
#### `set_surface_front_temperature(name : string, v: number)`
 Sets the `front_temperature` of the Surface called `name` to a value of `v`
#### `set_surface_back_temperature(i : int, v: number)`
 Sets the `back_temperature` of the Surface in index `i` to a value of `v`
#### `set_surface_back_temperature(name : string, v: number)`
 Sets the `back_temperature` of the Surface called `name` to a value of `v`