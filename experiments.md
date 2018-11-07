# Experiments

List of tests done on the code.

Used for reference, why certain decisions were made in code.

## Experiment 1

### Question

When running `a + &b`, but `b` memory array length is bigger than `a`, what is faster?

1. Reuse the memory from `a`, and add `b`.
2. Clone `b`, and add `a`.

### Test

Compare:

* `a` > `b` with memory reuse of `a`.
* `a` < `b` with memory reuse of `a`.
* `a` < `b` with cloning `a`.

### Results

20000000 runs in playground stable (Rust 1.30).

#### `a` > `b` with memory reuse of `a`.

    2.625213439s
    3.045003203s
    2.735490771s
    Average: 2.801902471s

#### `a` < `b` with memory reuse of `a`.

    7.092064169s
    > 8 sec
    > 8 sec
    Average: > 8 sec

#### `a` < `b` with cloning `a`.
    3.527505196s
    3.277101392s
    4.738751613s
    Average: 3.847786067

### Conclusion

We should only reuse the memory if the reusable variable is bigger.

If the only reusable variable is smaller, clone the bigger variable instead.
