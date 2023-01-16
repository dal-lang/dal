![DAL](https://avatars.githubusercontent.com/u/117913616?s=400&u=b56d1407d15a00a4ae8e30f9515263271320e3a7&v=4)

Dal is a general-purpose programming language designed to be simple yet powerfull.

## Building
### Status
![build](https://github.com/dal-lang/dal/actions/workflows/cmake.yml/badge.svg)
[![CircleCI](https://circleci.com/gh/dal-lang/dal.svg?style=svg)](https://circleci.com/gh/dal-lang/dal)

### Dependencies and Tools
- clang 15.0.5
- LLVM 15.0.5

### Debug / Development build
```
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
make
```

### Goals
- Do not depend on libc unless explicitly linked.
- Optionall standard library and will statically linked if used.
- Generics so that one can write efficient data structures that work for any data type.
- Ability to run arbitrary code at compile time and generate code.
- Provide build system and package manager.
- Easy integration with `C` library.

## License
Dal is available under MIT license, and comes with a humble request: use it to make software better serve the needs of end-users.
