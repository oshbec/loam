# Decisions

A log of decisions made about the project.

## `Position` values as an `f64`

- `f64` is super precise
- It might be overkill for this type of thing though
- I don't want to worry about evaluating when f32 won't work at a global scale when calculating things like distance, and I'd rather not be converting stuff back and forth
- Not terribly concerned about memory use
- Might revisit in the future, but it's simpler for now to get more precision by default
- Summary: going for simplicity in our choice of float, and it's good enough

## Constructing `Position` from 2- or 3-member tuples with `impl From`

- A `::new` constructur would be odd; sometimes there are two, other times there are three values for a `Position`
- Could be solved with a trait that gets implemented on `(f64, f64)` and `(f64, f64, f64)` which would then be accepted as a constructor argument; this is too complicated

```rs
let position: Position = (1.0, 2.0).into();
```

## Some standard derivations for `Position`: `Debug`, `PartialEq`, `Clone`, `Copy`

- `Debug`: We want to print the thing for debugging
- `PartialEq`: We want to compare it to other structs (and NOT `Eq` because that's not implemented for `f64`)
- `Clone`: Allow the thing to be cloned
- `Copy`: It's a really simple struct, so we don't need to worry about transferring ownership

## Deriving `Copy` for `Point`

- A point just contains one `Position` in `coordinates`, which itself derives `Copy`

## Not deriving `Copy` for things like `LineString`

- You can't... it's allocated to the heap, in this case by means of `Vec<Position>`

## Everthing's based on WGS84 as a coordinate reference system

- Basing this all heavily on GeoJSON
- GeoJSON only [officially supports](https://datatracker.ietf.org/doc/html/rfc7946#section-4) WGS84
