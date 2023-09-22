# Rust binding for C++

This is a mult-workspace solution that includes a rather simple C++ project as well as rust project.
The objective is to demonstrate how one might generate Rust bindings for a C++ project based on an installed header.

## Build the cpp project

```bash
cd lmvs-cpp 
cmake -B build -S . && cd build && make
sudo make install
```

This will build and install the public header files and static libraries to the following locations respectively:

-- Up-to-date: /usr/local/include/lmvs/add.h
-- Up-to-date: /usr/local/include/lmvs/div.h
-- Installing: /usr/local/lib/lmvs/liblmvs.a




## Build the Rust bindings and sample application

```bash
cd lmvs-rust
cargo build --release
cargo run 
```
