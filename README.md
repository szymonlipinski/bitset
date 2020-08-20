
An implementation of BitSet in Rust.

All is fully tested, including random tests with quickcheck.


# Benchmarks

I made simple benchmarks using `cargo bench`.

They check the speed of some simple bit operations on bitsets implemented as:

- a single u64 variable
- an array with a single u64 variable
- a list with a single u64 variable

There are two versions of each function checked: inlined and not inlined.


## Benchmark Results

The results are at [report/Bits/report/index.html](report/Bits/report/index.html).

The interesting parts are:

- the speed of the not inlined vector is 4 times smaller than the speed of the rest of the functions
- the speed of the inlined vector implementation is the same as the speed of the single variable one
