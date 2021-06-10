`friendly` is a human-readable display library.

This module provides a convenient, uniform way to display various types of quantities
in approximate, human-readable format.  For example:

```
# use friendly::bytes;
let kb = format!("{}", bytes(13200));
assert_eq!(kb.as_str(), "12.89 KiB")
```

The various functions provide quick ways to wrap values and types in the appropriate
objects to facilitate their display. Types such as [Quantity] then provide methods to
further customize this presentation.

## Features

This crate supports some features:

- `chrono` — enables support for types from the Chrono crate (currently just [chrono::Duration])
