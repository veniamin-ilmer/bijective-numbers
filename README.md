# Bijective Numbers Rust Crate

This is an unusual math rust crate.

On one hand, it is an experiment to see how well Rust optimizes code.

On the otherhand, it is meant to be a full-fledged math library.

This math library is be able to handle any size integer. A "big integer".

It can add these integers together, and provide the result.

This library probably won't beat other math libraries in a speed test. (Though you are welcome to test it out.)

It is not optimized to use your processor's math capabilities.

Instead, it converts all numbers into a binary bijective numeration. This bijective numeration is stored as a Bit/Bool Vector in Rust.

Addition, or any other simple math operation, involves simply a match of bits. This simulates how the CPU adds numbers on the chip level, except here it's using a bijective number array.

This project will continue to use a Bit/Bool Vector to save all numbers. It will never try to optimize the functionality by using the processor's math opcodes instead.

I will otherwise add more mathematical functionality to the crate over time.

The efficiency of this code entirely depends on how Rust/LLVM handles a Bit/Bool Vector. I hence hope that the code's efficiency will increase over time due to enhancements to Rust/LLVM.

That is why this code is considered an experiment to see how well Rust optimizes code. The code is relatively simple, so it is ripe for optimization.
