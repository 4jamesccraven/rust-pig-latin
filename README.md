Why?
----

This is included in [The Book](https://doc.rust-lang.org/beta/book/ch08-03-hash-maps.html#summary) as challenge
that was probably intended to be simple, but I kept thinking about all the odd edge-cases where it would be a bad
idea to rely on just ascii vowels. So I wrote this! It makes use of the
[Carnegie Mellon Pronouncing Dictionary](https://github.com/cmusphinx/cmudict) to determine what is pronounced as
a vowel, if possible, or otherwise falls back to ascii. 

Install
-------

Download or clone and run in the source directory:
```
cargo build --release
./target/release/pig_latin [file]
```
Doing so will print the same file to stdout, but in Pig Latin!

Limitations
-----------

This doesn't cover all edge cases, I mostly just wanted to learn some rust. Notably, if you run it on the benchmark
there will be some punctuation or numbers that just get treated as words/consonants and do weird things.
