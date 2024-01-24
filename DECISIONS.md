# Decisions

A log of decisions made about the project.

## `Position` values as an `f32` instead of `f64`

- In short, `f32` is precise enough; more precise than the reading you'd get from a GPS sensor
- Something like `f64` would be overkill, unless you had a sub-millimeter precision use case
- It's nice to save memory, especially when geometries might be made of *many* positions

## Constructing `Position` from 2- or 3-member tuples with `impl From`

- A `::new` constructur would be odd; sometimes there are two, other times there are three values for a `Position`
- Could be solved with a trait that gets implemented on `(f32, f32)` and `(f32, f32, f32)` which would then be accepted as a constructor argument; this is too complicated

```rs
let position: Position = (1.0, 2.0).into();
```

## Some standard derivations for `Position`: `Debug`, `PartialEq`, `Clone`, `Copy`

- `Debug`: We want to print the thing for debugging
- `PartialEq`: We want to compare it to other structs (and NOT `Eq` because that's not implemented for `f32`)
- `Clone`: Allow the thing to be cloned
- `Copy`: It's a really simple struct, so we don't need to worry about transferring ownership

## Deriving `Copy` for `Point`

- A point just contains one `Position` in `coordinates`, which itself derives `Copy`

## Not deriving `Copy` for things like `LineString`

- You can't... it's allocated to the heap, in this case by means of `Vec<Position>`
