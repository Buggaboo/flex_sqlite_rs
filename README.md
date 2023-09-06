# flex_sqlite_rs
PoC: Sqlite3 extension to collect sql rows, and spit out flexbuffers, with rust

## About
This is part of an experiment with sqlite3 extensions in rust and [flex-](https://github.com/Buggaboo/flex_sqlite_rs) and [flatbuffers](https://github.com/Buggaboo/flat_sqlite_rs).

## Conclusion
It seems that the base64 payload of flatbuffers are a lot larger than flexbuffers
All we need now is a magic attribute that generates the packing and unpacking code for flexbuffers.

## TODO
- [ ] Create attribute to generate unpacking function for arbitrary columns
- [ ] Benchmark flex vs flat packing speed
- [ ] Benchmark flex vs flat unpacking speed
