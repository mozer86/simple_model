# Surface

  A fixed surface in the building (or surroundings). This can be of
  any Construction, transparent or not.


```rs
Surface {
   name : string,
   vertices : Polygon3D,
   construction : Construction,
   front_boundary : Boundary, // optional,
   front_receives_sun : boolean, // optional,
   back_receives_sun : boolean, // optional,
   back_boundary : Boundary, // optional,
}
```



#### name

  The name of the surface




#### vertices

  An array of Numbers representing the vertices of the
  surface. The length of this array must be divisible by 3.




#### construction

  The index of the construction in the SimpleModel\'s
  Construction array    




#### front_boundary (*optional*)

  A reference to the Boundary in front of the Surface




#### front_receives_sun (*optional*)





#### back_receives_sun (*optional*)





#### back_boundary (*optional*)

  A reference to the Boundary in back of the Surface






## API Access

```rs
// by name
let my_surface = surface(string);
// by index
let my_surface = surface(int);
```



## API

The following properties are available for simulating control algorithms

| Property | Getter | Setter |
|----------|--------|--------|

| `front_temperature` | Yes   | Research mode |
| `back_temperature` | Yes   | Research mode |
| `front_convection_coefficient` | Yes   | Research mode |
| `back_convection_coefficient` | Yes   | Research mode |
| `front_convective_heat_flow` | Yes   | Research mode |
| `back_convective_heat_flow` | Yes   | Research mode |
| `front_incident_solar_irradiance` | Yes   | Research mode |
| `back_incident_solar_irradiance` | Yes   | Research mode |
| `front_ir_irradiance` | Yes   | Research mode |
| `back_ir_irradiance` | Yes   | Research mode |