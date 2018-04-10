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

 *  Make sure that `cargo fuzz` works for you locally.

 *  Make the file `seasoned-software.sh` in the root of your
    repository. This file should specify how to compile all the `cargo
    fuzz` targets in your project, and then it should call
    `register-binary` for each target you want to use as fuzz harness.

    In this project we only have one target, `quicksort`, thus our
    script is just:

    ```
    #!/bin/bash

    # Cargo-fuzz needs nightly rust, so switch this project to nightly
    rustup override set nightly

    # Build the quicksort fuzz target in release mode, and run it once
    cargo fuzz run quicksort --release -- -help=1

    # Find the executable for the fuzz target
    EXE="$(find fuzz/target -iname quicksort -executable)"

    # Register the fuzz target
    register-binary "quicksort" "$EXE"
    ```

    If you have multiple fuzz targets you can use the following
    snippet of code instead of the last four steps in the script:

    ```
    # Build and register all tests known to Cargo-fuzz.
    for t in $(cargo fuzz list|sed 's@\x1b[^m]*m@@g'); do
	   echo "Building test: $t"
	   cargo fuzz run $t --release -- -help=1
	   EXE="$(find fuzz/target -iname $t -executable)"

       echo "Registering: $EXE"
	   register-binary "$t" "$EXE"
    done
    ```

 *  Now go to <https://app.seasoned.software> and add your project.

    When you add your project you need to tell which git branch to use
    (`master` in our case) and which harness to use, the harness is
    the name of one of the binaries we registered with
    `register-binary`. In this project we only have one harness
    `quicksort`.
