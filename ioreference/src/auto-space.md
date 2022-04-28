# Space

  Represents a space within a building. This will
  often be a room, but it might also be half a room


```rs
Space {
   name : string,
   volume : number, // optional,
   infiltration : Infiltration, // optional,
   building : Building, // optional,
}
```



#### name

  The name of the space




#### volume (*optional*)

  Volume of the space




#### infiltration (*optional*)

  The infiltration in the space




#### building (*optional*)

  The importance of this space over time






## API Access

```rs
// by name
let my_space = space(string);
// by index
let my_space = space(int);
```



## API

The following properties are available for simulating control algorithms

| Property | Getter | Setter |
|----------|--------|--------|

| `dry_bulb_temperature` | Yes   | Research mode |
| `brightness` | Yes   | Research mode |
| `loudness` | Yes   | Research mode |
| `infiltration_volume` | Yes   | Research mode |
| `infiltration_temperature` | Yes   | Research mode |
| `ventilation_volume` | Yes   | Research mode |
| `ventilation_temperature` | Yes   | Research mode |