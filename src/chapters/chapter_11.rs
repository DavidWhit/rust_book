// Writing tests

/*
    *** SEE:
    lib.rs
    tests
        /test_integrate_mod
            /mod.rs
        /integration_test.rs

    use

    #[cfg(tests)] attribute to define tests entry point
    #[tests] to mark function as tests

    # run with cargo tests
    # cargo tests
    # cargo tests --help    -> options on the file
    # cargo tests -- --help -> options you can use after the first --

    *** Running tests in parallel or consecutively

    You can also run tests in parallel or consecutively
    Because the tests are running at the same time, make sure your tests don’t depend on each
    other or on any shared state, including a shared environment, such as the current working
    directory or environment variables.

    cargo tests -- --tests-threads=1

    Showing output
    cargo tests -- --show-output
    if functions print anything before return values

    Running particular tests
    tests


    Running single tests
    cargo tests fn_name

    Running multiple of like tests by name
    EX two tests functions that being with add_
    cargo tests add


    Testing is thought by the Rust community as two categories:
    1. unit tests
    2. integration tests

    For unit tests they should be in the src directory.
    The conventions is to create a module named tests in each file to contain the tests functions
    and to annotate the module with cfg(tests)

    The #[cfg(tests)] annotation on the tests module tells Rust to compile an run code
    when you execute cargo tests

    *** Testing Private Functions

    Rust privacy rules do allow you to tests private functions
    Consider the code in listing 11-12 with private functions see lib.rs

   *** Integration Tests
   These are entirely external to your library. They use your library in the same way
   any other code would, which means they can only call functions that are part of your library's
   public API. Their purpose is to tests whether many parts of your library work together correctly.

   The tests Directory Convention
   Create tests directory at the top level of the project next to src
   ---- cargo knows to look here -----
 */