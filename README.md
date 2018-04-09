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
  fuzz` targets in your project, and then it should call
  `register-binary` for each target you want to use as fuzz harness.

  In this project we only have one target: `quicksort`

* Now go to <https://app.seasoned.software> and add your project.

  When you add your project you need to tell which git branch to use
  (`master` in our case) and which harness to use, the harness is the
  name of one of the binaries we registered with `register-binary`. In
  this project we only have one harness `quicksort`.
