# UTF-16

Work with UTF-16 in Rust.

## Differences to other crates

- `utf16string` - `realhydroper-utf16` uses code units as `u16` instead of octets for indexing strings, as opposed to `utf16string`.

## Converting offsets between UTF-8 and UTF-16

Use the `utils` submodule for converting between offset encodings:

```rust
use realhydroper_utf16::{Utf16String, utils::*};

let utf8string = "a\u{10FFFF}b\u{10000}";
let utf16string = Utf16String::from(utf8string);

// (start: usize, end: usize)
assert_eq!(two_utf16_offsets_as_utf8_offsets(utf8string, &utf16string, 3, 4), (5, 6));
assert_eq!(two_utf8_offsets_as_utf16_offsets(&utf16string, utf8string, 5, 6), (3, 4));
```

*More efficiency*

The above is inefficient for large strings. If you have line slices, do something like the following to convert from UTF-16 to UTF-8:

> Note: adjust the line numbers correctly depending on whether they are "zero" based or "one" based.

```rust
use realhydroper_utf16::{Utf16String, utils::*};

fn utf16_range_to_utf8_offsets(src: &Utf8SourceText, range: Utf16Range) -> (usize, usize) {
    let start_line_offset = src.get_line_offset((range.start.line as usize) + 1).unwrap();
    let end_line_offset = src.get_line_offset((range.end.line as usize) + 1).unwrap();

    let start_line_offset_next = src.get_line_offset((range.start.line as usize) + 2).unwrap_or(src.contents.len());
    let end_line_offset_next = src.get_line_offset((range.end.line as usize) + 2).unwrap_or(src.contents.len());

    let start_line_utf8 = &src.contents[start_line_offset..start_line_offset_next];
    let end_line_utf8 = &src.contents[end_line_offset..end_line_offset_next];

    let start_line_utf16 = Utf16String::from(start_line_utf8);
    let end_line_utf16 = Utf16String::from(end_line_utf8);

    let first_offset = utf16_offset_as_utf8_offset(start_line_utf8, &start_line_utf16, range.start.character as usize);
    let last_offset = utf16_offset_as_utf8_offset(end_line_utf8, &end_line_utf16, range.end.character as usize);

    (start_line_offset + first_offset, end_line_offset + last_offset)
}
```

Do something like the following to convert from UTF-8 to UTF-16:

> Note: adjust the line numbers correctly depending on whether they are "zero" based or "one" based.

```rust
use realhydroper_utf16::{Utf16String, utils::*};

fn utf8_loc_to_utf16_range(loc: &Utf8Location) -> Utf16Range {
    let src = loc.source();
    let first_line = loc.first_line_number();
    let last_line = loc.last_line_number();

    let start_line_offset = src.get_line_offset(first_line).unwrap();
    let end_line_offset: usize = src.get_line_offset(last_line).unwrap();

    let start_line_offset_next = src.get_line_offset(first_line + 1).unwrap_or(src.text().len());
    let end_line_offset_next = src.get_line_offset(last_line + 1).unwrap_or(src.text().len());

    let start_line_utf8 = &src.text()[start_line_offset..start_line_offset_next];
    let end_line_utf8 = &src.text()[end_line_offset..end_line_offset_next];

    let start_line_utf16 = Utf16String::from(start_line_utf8);
    let end_line_utf16 = Utf16String::from(end_line_utf8);

    // Start offset from start line
    let start_offset = utf8_offset_as_utf16_offset(&start_line_utf16, &start_line_utf8, loc.first_offset() - start_line_offset);
    // End offset from end line
    let end_offset = utf8_offset_as_utf16_offset(&end_line_utf16, &end_line_utf8, loc.last_offset() - end_line_offset);

    Utf16Range {
        start: Position {
            line: (first_line - 1) as u32,
            character: start_offset as u32,
        },
        end: Position {
            line: (last_line - 1) as u32,
            character: end_offset as u32,
        },
    }
}
```
