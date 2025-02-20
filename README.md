# UTF-16 string

UTF-16 string types for Rust.

Differently from the `utf16string` crate, this implementation uses natively-sized UTF-16 code units rather than octets.

## Converting offsets between UTF-8 and UTF-16

Use the `utils` submodule for converting between offset encodings:

```rust
use realhydroper_utf16string::{Utf16String, utils::*};

let utf8string = "a\u{10FFFF}b\u{10000}";
let utf16string = Utf16String::from(utf8string);

assert_eq!(two_utf16_offsets_as_utf8_offsets(utf8string, &utf16string, 3, 4), (5, 6));
assert_eq!(two_utf8_offsets_as_utf16_offsets(&utf16string, utf8string, 5, 6), (3, 4));
```