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

The following properties are available for simulating control algorithms


| Property | Getter | Setter |
|----------|--------|--------|
| `front_temperature` | Yes   | Research mode |
| `back_temperature` | Yes   | Research mode |