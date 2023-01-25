## Contributing guidelines

> __Note__ \
> This guide is not complete yet and will be updated as the project progresses.

### Table of Contents

- [Introduction](#introduction)
- [Reporting Bugs](#reporting-bugs)
- [Contributing Code](#contributing)

### Introduction

Thank you for your interest in contributing to Dal. This document is a set of guidelines for contributing to Dal. These
are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull
request.

#### Code structure

The code is divided into several parts:

- `include` - contains the header files for the compiler.
    - `include/cli` - contains the header files for the command line interface.
    - `include/core` - contains the header files for the core of the compiler.
    - `include/fmt` - contains the header files for the formatter.
- `src` - contains the source files for the compiler.
    - `src/cli` - contains the source files for the command line interface.
    - `src/core` - contains the source files for the core of the compiler.
    - `src/fmt` - contains the source files for the formatter.
- `test` - contains the test files for the compiler.

### Reporting Bugs

Since Dal is still in early development, there are bound to be bugs. If you find a bug, please report it in the
[issue tracker](https://github.com/dal-lang/dal/issues). Please include the following information in your bug report:

- The version of Dal you are using.
- The operating system you are using.
- The steps to reproduce the bug.
- The expected behavior.
- The actual behavior.
- If possible, a minimal code example that reproduces the bug.

If the bug is a security vulnerability, please have a look at the [security policy](./markdown/security.md).

### Contributing

You can contribute to Dal by writing code, fixing bugs, resolving issues, writing documentation, or by translating
the documentation to other languages.

#### Contributing Code

If you want to contribute code, please follow these steps:

1. Fork the repository.
2. Create a new branch for your changes.
3. Make your changes.
4. Make sure the tests pass.
5. Create a pull request.

Other than that, there are no strict rules for contributing code. You can use any style you want, as long as the code
is readable and easy to understand by other contributors.

#### Building the compiler

To build the compiler, you need to have the following installed:

- CMake
- A C and C++ compiler (In our CI we use clang version 15.0.5), but any compiler that supports C++17 should work.
- LLVM 15.0.5

To build the compiler, run the following commands:

```bash
mkdir build
cd build
cmake ..
cmake --build .
```

#### Running the tests

To run the tests, you must pass the `DAL_USE_TEST` option to CMake:

```bash
cmake -DDAL_USE_TEST=ON ..
```

and then run the tests:

```bash
ctest
```

## Code of Conduct

By participating in this project, you agree to abide by the [code of conduct](./markdown/code_of_conduct.md).