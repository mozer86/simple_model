# Boundary

  Represents the boundary of a `Surface`
 
  By default (i.e., if no boundary is assigned to a Surface),
  the boundary will be assumed to be outside.


## Ground

```rs
Boundary::Ground()
```

  The Surface is in contact with the Ground


## Space

```rs
Boundary::Space(Space)
```

  The Surface leads to another surface


