use ahash::AHasher;

use std::hash::{Hash, Hasher};

/**
Get a hash for a value. This is handy for anytime you need a random-ish, but
constant, value based on some other value. One good usecase is
variagated tilesets: pass in the tile's [`ICoord`] position to this function
and use it as a selector on the tile variants.

This isn't guaranteed to be the same across compiles or restarts,
but it will be the same for a given input value across one run of a program.

```
# use cogs_gamedev::hash::hashcode;

assert_eq!(hashcode(&10i32), hashcode(&10i32));
assert_ne!(hashcode(&10i32), hashcode(&600i32));

```

[`ICoord`]: crate::grids::ICoord;
*/
pub fn hashcode<H: Hash>(hashee: &H) -> u64 {
    let mut hasher = AHasher::default();
    hashee.hash(&mut hasher);
    hasher.finish()
}
