# Space

  Represents a space within a building. This will
  often be a room, but it might also be half a room


```rs
Space {
	name : string
	volume : number   // Optional
	infiltration : Infiltration   // Optional
}
```

## Fields



#### name

  The name of the space




#### volume  (*optional*)

  Volume of the space




#### infiltration  (*optional*)

  The infiltration in the space








## API

The following functions are available for simulating control algorithms
### Standard use



#### `space_dry_bulb_temperature(i : int)`
 Gets the `dry_bulb_temperature` of the Space in index `i`
#### `space_dry_bulb_temperature(name : string)`
 Gets the `dry_bulb_temperature` of the Space called `name`
#### `space_brightness(i : int)`
 Gets the `brightness` of the Space in index `i`
#### `space_brightness(name : string)`
 Gets the `brightness` of the Space called `name`
#### `space_loudness(i : int)`
 Gets the `loudness` of the Space in index `i`
#### `space_loudness(name : string)`
 Gets the `loudness` of the Space called `name`
#### `space_infiltration_volume(i : int)`
 Gets the `infiltration_volume` of the Space in index `i`
#### `space_infiltration_volume(name : string)`
 Gets the `infiltration_volume` of the Space called `name`
#### `space_infiltration_temperature(i : int)`
 Gets the `infiltration_temperature` of the Space in index `i`
#### `space_infiltration_temperature(name : string)`
 Gets the `infiltration_temperature` of the Space called `name`
#### `space_ventilation_volume(i : int)`
 Gets the `ventilation_volume` of the Space in index `i`
#### `space_ventilation_volume(name : string)`
 Gets the `ventilation_volume` of the Space called `name`
#### `space_ventilation_temperature(i : int)`
 Gets the `ventilation_temperature` of the Space in index `i`
#### `space_ventilation_temperature(name : string)`
 Gets the `ventilation_temperature` of the Space called `name`
### Research mode only

#### `set_space_dry_bulb_temperature(i : int, v: number)`
 Sets the `dry_bulb_temperature` of the Space in index `i` to a value of `v`
#### `set_space_dry_bulb_temperature(name : string, v: number)`
 Sets the `dry_bulb_temperature` of the Space called `name` to a value of `v`
#### `set_space_brightness(i : int, v: number)`
 Sets the `brightness` of the Space in index `i` to a value of `v`
#### `set_space_brightness(name : string, v: number)`
 Sets the `brightness` of the Space called `name` to a value of `v`
#### `set_space_loudness(i : int, v: number)`
 Sets the `loudness` of the Space in index `i` to a value of `v`
#### `set_space_loudness(name : string, v: number)`
 Sets the `loudness` of the Space called `name` to a value of `v`
#### `set_space_infiltration_volume(i : int, v: number)`
 Sets the `infiltration_volume` of the Space in index `i` to a value of `v`
#### `set_space_infiltration_volume(name : string, v: number)`
 Sets the `infiltration_volume` of the Space called `name` to a value of `v`
#### `set_space_infiltration_temperature(i : int, v: number)`
 Sets the `infiltration_temperature` of the Space in index `i` to a value of `v`
#### `set_space_infiltration_temperature(name : string, v: number)`
 Sets the `infiltration_temperature` of the Space called `name` to a value of `v`
#### `set_space_ventilation_volume(i : int, v: number)`
 Sets the `ventilation_volume` of the Space in index `i` to a value of `v`
#### `set_space_ventilation_volume(name : string, v: number)`
 Sets the `ventilation_volume` of the Space called `name` to a value of `v`
#### `set_space_ventilation_temperature(i : int, v: number)`
 Sets the `ventilation_temperature` of the Space in index `i` to a value of `v`
#### `set_space_ventilation_temperature(name : string, v: number)`
 Sets the `ventilation_temperature` of the Space called `name` to a value of `v`