# xlrd

A Rust port of Python's `xlrd` ?

The library is a pure Rust library used for reading data and formatting information from Excel files in the historical `.xls` BIFF8 format.

It focuses solely on reading basic data and formatting information, and store them in the `umya-spreadsheet::Spreadsheet` structure, so you can easily modify and save it as `.xlsx`.

## Unsupported Features
- Password-protected files
- Embedded Objects (Chars, Macros, Pictures, Worksheets etc.)
- Formulas
- VBA modules
- Comments
- Hyperlinks
- Autofilters, advanced filters, pivot tables, conditional formatting, data validation

## Install

Run the following Cargo command in your project directory:
```sh
cargo add xlrd

# If need to view the reading process
cargo add xlrd --features tracing

# When need to modify the data or formatting, may also need to add
cargo add umya-spreadsheet
```

Or add the following line to your Cargo.toml:
```toml
xlrd = "0.1"

# If need to view the reading process
xlrd = { version = "0.1", features = ["tracing"] }

# When need to modify the data or formatting, may also need to add
umya-spreadsheet = "2"
```

## Example

It's very simple, `xlrd` has only 3 functions:
- `xlrd::open`
- `xlrd::save`
- `xlrd::xls2xlsx`

```rust
// Only just when enable the `tracing` feature
tracing_subscriber::fmt::init();

let mut workbook = xlrd::open("path/from/your/file.xls").unwrap();

// After modify the data or formating...
xlrd::save(&workbook, "path/to/your/file.xlsx").unwrap();

// Or just convert to `.xlsx`, automatically save to the origin directory
let xlsx_path = xlrd::xls2xlsx("path/from/your/file.xls").unwrap();
```

## Reference

- [umya-spreadsheet](https://crates.io/crates/umya-spreadsheet)
- [xlrd](https://pypi.org/project/xlrd)
- [xls2xlsx](https://pypi.org/project/xls2xlsx/)

## License

MIT