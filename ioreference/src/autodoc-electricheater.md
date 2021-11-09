# ElectricHeater



```rs
ElectricHeater {
	name : string
	target_space : Space   // Optional
	max_heating_power : number   // Optional
}
```

## Fields



#### name

  The name of the system




#### target_space  (*optional*)

  The [`Space`] that this [`ElectricHeater`] heats and/or
  cools




#### max_heating_power  (*optional*)

  Max heating power








## API

The following properties are available for simulating control algorithms


| Property | Getter | Setter |
|----------|--------|--------|
| `power_consumption` | Yes   | Yes |