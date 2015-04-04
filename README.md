# txtdist
A library for mesuring the distance between two strings.

Currently has the Damerau-Levenschtein algorithm, so only one function.

```rust
extern crate txtdist;
use txtdist::damerau_levenshtein;

let distance = damerau_levenshtein("some string", "some other string");
assert_eq!(distance, 6)
```
