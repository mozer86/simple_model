# Fenestration

  A surface that can potentially be opened and closed.
  It can be of any Construction and it does not need to be
  a hole in another surface.


```rs
Fenestration {
	name : string
	vertices : [ number, ...]
	construction : Construction
	operation_type : FenestrationPositions
	fenestration_type : FenestrationType
	front_boundary : Boundary   // Optional
	back_boundary : Boundary   // Optional
}
```

## Fields



#### name

  The name of the sub surface




#### vertices

  An array of Numbers representing the vertices of the 
  surface. The length of this array must be divisible by 3.




#### construction

  The index of the Construction object in the
  constructions property of the SimpleModel object    




#### operation_type

  The opportunity for operating the Fenestration




#### fenestration_type

  It it a window or a door, or...?




#### front_boundary  (*optional*)

  A reference to the Boundary in front of the Fenestration




#### back_boundary  (*optional*)

  A reference to the Boundary in back of the Fenestration








## API

The following functions are available for simulating control algorithms
### Standard use

#### `fenestration_open_fraction(i : int)`
 Gets the `open_fraction` of the Fenestration in index `i`
#### `fenestration_open_fraction(name : string)`
 Gets the `open_fraction` of the Fenestration called `name`

#### `set_fenestration_open_fraction(i : int, v: number)`
 Sets the `open_fraction` of the Fenestration in index `i` to a value of `v`
#### `set_fenestration_open_fraction(name : string, v: number)`
 Sets the `open_fraction` of the Fenestration called `name` to a value of `v`

#### `fenestration_front_temperature(i : int)`
 Gets the `front_temperature` of the Fenestration in index `i`
#### `fenestration_front_temperature(name : string)`
 Gets the `front_temperature` of the Fenestration called `name`
#### `fenestration_back_temperature(i : int)`
 Gets the `back_temperature` of the Fenestration in index `i`
#### `fenestration_back_temperature(name : string)`
 Gets the `back_temperature` of the Fenestration called `name`
### Research mode only

#### `set_fenestration_front_temperature(i : int, v: number)`
 Sets the `front_temperature` of the Fenestration in index `i` to a value of `v`
#### `set_fenestration_front_temperature(name : string, v: number)`
 Sets the `front_temperature` of the Fenestration called `name` to a value of `v`
#### `set_fenestration_back_temperature(i : int, v: number)`
 Sets the `back_temperature` of the Fenestration in index `i` to a value of `v`
#### `set_fenestration_back_temperature(name : string, v: number)`
 Sets the `back_temperature` of the Fenestration called `name` to a value of `v`