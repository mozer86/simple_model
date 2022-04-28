# Fenestration

  A surface that can potentially be opened and closed.
  It can be of any Construction and it does not need to be
  a hole in another surface.


```rs
Fenestration {
   name : string,
   vertices : Polygon3D,
   construction : Construction,
   operation_type : FenestrationPositions,
   fenestration_type : FenestrationType,
   front_boundary : Boundary, // optional,
   back_boundary : Boundary, // optional,
}
```



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




#### front_boundary (*optional*)

  A reference to the Boundary in front of the Fenestration




#### back_boundary (*optional*)

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

| `front_temperature` | Yes   | Research mode |
| `back_temperature` | Yes   | Research mode |
| `open_fraction` | Yes   | Yes |
| `front_convection_coefficient` | Yes   | Research mode |
| `back_convection_coefficient` | Yes   | Research mode |
| `front_convective_heat_flow` | Yes   | Research mode |
| `back_convective_heat_flow` | Yes   | Research mode |
| `front_incident_solar_irradiance` | Yes   | Research mode |
| `back_incident_solar_irradiance` | Yes   | Research mode |
| `front_ir_irradiance` | Yes   | Research mode |
| `back_ir_irradiance` | Yes   | Research mode |