# IdealHeaterCooler

  An ideal Heating and Cooling device, with a COP of 1.
 


```rs
IdealHeaterCooler {
   name : string,
   target_spaces : [Space, ...],
   max_heating_power : number, // optional,
   max_cooling_power : number, // optional,
}
```



#### name

  The name of the system




#### target_spaces

  The `Space`s that this `IdealHeaterCooler` heats and/or
  cools




#### max_heating_power (*optional*)

  Max heating power




#### max_cooling_power (*optional*)

  Max cooling power








## API

The following properties are available for simulating control algorithms

| Property | Getter | Setter |
|----------|--------|--------|

| `power_consumption` | Yes   | Yes |