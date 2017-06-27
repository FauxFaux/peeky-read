## peeky-read

A single-struct library providing `PeekyRead`.

`PeekyRead` takes ownership of an `io::Read`,
and provides a `check_eof() -> io::Result<bool>`.

This is accomplished by actually reading a single
byte from the underlying reader. The byte is stored,
and returned automatically by the next `read()`, so
nothing is lost, and the transition should be transparent.
