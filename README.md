# abx2xml-rs

This is the Modified Version Of https://github.com/rhythmcache/abx2xml-rs/ with XML formatting added.



## CLI Installation
```bash
cargo install --git https://github.com/ThisColdBlueSky/abx2xml-rs/
```



## CLI Usage
```bash
Usage: abx2xml [OPTIONS] <input> [output]

Arguments:
  <input>   Input file path (use '-' for stdin)
  [output]  Output file path (use '-' for stdout)

Options:
      --no-format  Skip XML formatting (output raw XML)
  -i, --in-place  Overwrite input file with converted output
```



### Sources
- [xml2abx](https://github.com/rhythmcache/xml2abx-rs)

- [BinaryXmlPullParser.java](https://cs.android.com/android/platform/superproject/+/master:frameworks/base/core/java/com/android/internal/util/BinaryXmlPullParser.java;bpv=0)


### License
This project is licensed under
- Apache License, Version 2.0

