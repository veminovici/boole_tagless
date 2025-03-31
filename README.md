# Boole Tagless

A Rust project demonstrating the evolution of tagless final interpreters for simple expressions, showcasing different implementation approaches and their trade-offs.

## Overview

This project implements boolean expressions using the tagless final pattern, progressing through multiple iterations to demonstrate different approaches and their implications. Each implementation (`tagless00.rs` through `tagless05.rs`) builds upon the previous one, introducing new concepts and improvements.

### Implementation Progression

- `tagless00.rs`: Basic tagless final implementation
- `tagless01.rs`: Adding type-level improvements
- `tagless02.rs`: Introducing higher-kinded types
- `tagless03.rs`: Adding type-level constraints
- `tagless04.rs`: Implementing type-level operations
- `tagless05.rs`: Advanced type-level programming

## Prerequisites

- Rust 1.75 or later (2024 edition)
- Cargo (comes with Rust)
- Understanding of Rust's type system and functional programming concepts

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/boole_tagless.git
cd boole_tagless
```

2. Build the project:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

## Learning Resources

- [Tagless Final Pattern](https://okmij.org/ftp/tagless-final/)
- [Rust Type System](https://doc.rust-lang.org/book/ch19-00-advanced-types.html)
- [Higher-Kinded Types in Rust](https://github.com/rust-lang/rfcs/blob/master/text/0324-re-rebalancing-coherence.md)
- [Efficient, Extensible, Expressive: Typed Tagless Final Interpreters in Rust](https://getcode.substack.com/p/efficient-extensible-expressive-typed)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the tagless final pattern in functional programming
- Built with Rust and Cargo
- Special thanks to the Rust community for their work on type-level programming
