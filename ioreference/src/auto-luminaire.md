# Luminaire

  A Luminaire


```rs
Luminaire {
   name : string,
   max_power : number, // optional,
   target_space : Space, // optional,
}
```



#### name

  The name of the Luminaire




#### max_power (*optional*)

  The maximum power consumption




#### target_space (*optional*)

  The space in which the space is located
 
  While this value is might not be relevant for
  e.g., lighting calculations, this is necessary for
  thermal simulations, in which the heat disipated by
  a luminaire will be disipated into the air of a thermal
  zone. So, if this is an exterior luminaire or if no thermal
  calculation is performed, this can be left empty.






## API Access

```rs
// by name
let my_luminaire = luminaire(string);
// by index
let my_luminaire = luminaire(int);
```



## API

The following properties are available for simulating control algorithms

| Property | Getter | Setter |
|----------|--------|--------|

| `power_consumption` | Yes   | Yes |