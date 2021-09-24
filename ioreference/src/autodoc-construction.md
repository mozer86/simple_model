# Construction

  An object representing a multilayer
  Construction; that is to say, an array of
  Materials


```rs
Construction {
	name : string
	layers : [ Rc, ...] 
}
```

## Fields



### name

  The name of the Construction object.
  Must be unique within the model




### layers

  The indices of the Material objects in the
  materials property of the Building object




