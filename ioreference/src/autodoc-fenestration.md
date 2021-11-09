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






## API Access

```rs
// by name
let my_fenestration = fenestration(string);
// by index
let my_fenestration = fenestration(int);
```



## API

The following properties are available for simulating control algorithms


| Property | Getter | Setter |
|----------|--------|--------|
| `open_fraction` | Yes   | Yes |
| `front_temperature` | Yes   | Research mode |
| `back_temperature` | Yes   | Research mode |