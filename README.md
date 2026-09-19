# 🦀 Rustlings Solutions

My solutions to the [Rustlings](https://rustlings.rust-lang.org/) exercises, the official hands-on course for learning to read and write Rust code.

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust)
![Status](https://img.shields.io/badge/exercises-completed-brightgreen)

## About

Rustlings is a set of small exercises, each with broken code that you fix until it compiles and passes its tests. This repository contains my completed versions of all of them, working through topics from basic syntax to ownership, traits, lifetimes, and concurrency.

> **Heads up:** If you're doing Rustlings yourself, try each exercise on your own first. Most of the learning happens when you struggle with compiler errors. Use this repo to compare approaches *after* you've had a go, not as a shortcut.

## Topics Covered

| Area | Concepts |
|------|----------|
| Rust Basics | variables, functions, `if`, primitive types |
| Data | vectors, structs, enums, strings, hashmaps |
| Variables Ownership | move semantics, references and borrowing |
| Code Organization | modules, `use`, `pub` |
| Error handling | `Option`, `Result`, the `?` operator |
| Abstractions | generics, traits, lifetimes |
| Testing | unit tests, `assert!` macros |
| Functional | iterators, closures |
| Advanced | smart pointers (`Box`, `Rc`, `Arc`), threads, macros |
| Conversions | `From`, `Into`, `TryFrom`, `AsRef` |

## Repository Structure

```
.
├── exercises/        # My solved exercises, grouped by topic
│   ├───00_intro
│   ├───01_variables
│   ├───02_functions
│   ├───03_if
│   ├───04_primitive_types
│   ├───05_vecs
│   ├───06_move_semantics
│   ├───07_structs
│   ├───08_enums
│   ├───09_strings
│   ├───10_modules
│   ├───11_hashmaps
│   ├───12_options
│   ├───13_error_handling
│   ├───14_generics
│   ├───15_traits
│   ├───16_lifetimes
│   ├───17_tests
│   ├───18_iterators
│   ├───19_smart_pointers
│   ├───20_threads
│   ├───21_macros
│   ├───22_clippy
│   ├───23_conversions
│   └───quizzes
└───solutions
│   ├───00_intro
│   ├───01_variables
│   ├───02_functions
│   ├───03_if
│   ├───04_primitive_types
│   ├───05_vecs
│   ├───06_move_semantics
│   ├───07_structs
│   ├───08_enums
│   ├───09_strings
│   ├───10_modules
│   ├───11_hashmaps
│   ├───12_options
│   ├───13_error_handling
│   ├───14_generics
│   ├───15_traits
│   ├───16_lifetimes
│   ├───17_tests
│   ├───18_iterators
│   ├───19_smart_pointers
│   ├───20_threads
│   ├───21_macros
│   ├───22_clippy
│   ├───23_conversions
│   └───quizzes
├── Cargo.toml
├── .rustlings-state.txt
├── rust-analyzer.toml
└── README.md
```

## Doing Rustlings Yourself

1. Install Rust: <https://www.rust-lang.org/tools/install>
2. Install Rustlings: `cargo install rustlings`
3. Set up the exercises: `rustlings init`
4. Enter the new directory and run `rustlings`
5. Stuck? Use the built-in hint command (`h` in watch mode)

Full instructions are on the [official Rustlings site](https://rustlings.rust-lang.org/).

## What I Learned

I worked through these exercises after finishing [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) book, using them to put the concepts into practice through small, hands-on problems. The biggest lesson was learning to treat the compiler as a helpful partner rather than an obstacle.

Key areas I practiced:

- **Core syntax:** variables, data types, `if`, functions, enums, and structs
- **Memory model:** how data is stored on the stack and the heap
- **Ownership and borrowing:** moving values, borrowing references, and reading borrow-checker errors
- **Error handling:** using `Option`, `Result`, and the `?` operator
- **Testing:** writing unit tests to verify behavior
- **Generics, traits, and lifetimes:** building strongly typed, reusable code
- **Iterators:** processing collections idiomatically
- **Smart pointers:** using `Box`, `Rc`, and `Arc`
- **Concurrency:** spawning threads and communicating between them
- **Macros:** writing simple declarative macros
