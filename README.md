# Faiss-rs

[![Build Status](https://travis-ci.org/Enet4/faiss-rs.svg?branch=master)](https://travis-ci.org/Enet4/faiss-rs)

This project provides Rust bindings to Faiss, the state-of-the-art vector search similarity engine.

## Installing as a dependency

Currently, this crate does not build Faiss automatically for you. The dynamic library needs to be installed
manually to your system.

  1. Follow the instructions [here](https://github.com/facebookresearch/faiss/blob/master/INSTALL.md) to build Faiss.
  2. Afterwards, follow the instructions on [building the C API of Faiss](https://github.com/facebookresearch/faiss/blob/master/c_api/INSTALL.md). This will result in the dynamic library `faiss_c`, which needs to be installed in a place where your system will pick up (in Linux, try somewhere in the LD_LIBRARY_PATH environment variable, such as "/usr/lib", or try adding a new path to this variable).
  3. You are now ready to include this crate as a dependency:

```toml
[dependencies]
"faiss" = "0.1.0"
```

## License and attribution notice

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

This work is not affiliated with Facebook Research or the main Faiss project.