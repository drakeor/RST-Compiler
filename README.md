# Introduction
RST-Compiler is a rust-based small talk compiler I built for fun. It's very experimental and prone to breaking so I do not recommend using it to develop large-scale projects.
It can compile SmallTalk code into x86 assembly which can then by transformed into an executable by gcc. 

# Dependencies
* 64-bit Linux Machine (I used Debian)
* Rust 1.39+ (To compile the compiler)
* gcc 8.3 (May not work on certain versions of gcc. For example, I do not detect and automatically prepend "_" in front of function names based on gcc version)

