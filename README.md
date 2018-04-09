Fun with Fuzzing
================

This repository demonstrate two things:

* How use `cargo-fuzz` to fuzz test your Rust code.

* How to use the [Seasoned Software](https://seasoned.software) service
  to continuous fuzz test your code in the cloud.


How to use `cargo-fuzz`
-----------------------


How to use [Seasoned Software](https://seasoned.software)
---------------------------------------------------------

* Make sure that `cargo fuzz` works you locally

* Make file `seasoned-software.sh` in the root of your
  repository. This file should specify how to compile all the `cargo
  fuzz` targets in your project.
