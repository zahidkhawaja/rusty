# Contributing

If you're interested in contributing to Rusty, here are some useful tips!

First of all, don't hesitate to reach out with any questions!
This project is currently maintained by Zahid. Feel free to DM him on [Twitter]](https://twitter.com/chillzaza_) or shoot an email to [zahid@khawaja.ai]](mailto:zahid@khawaja.ai).

### Building Rusty

Rusty builds on stable Rust. Here are the steps to build it from source:

1. Navigate to the directory of your choice
0. Clone this repository with git.

   ```
   git clone https://github.com/zahidkhawaja/rusty.git
   ```
0. Navigate into the newly created `rusty` directory
0. Run `cargo build`

The resulting binary can be found in `rusty/target/debug/` under the name `rusty` or `rusty.exe`.

### Code Quality

Rust has some excellent tools to make this super easy!

#### Formatting Code with rustfmt

Before you make a pull Request to the project, please run it through the `rustfmt` utility.
This will format your code according to style guidelines and make things easier to maintain.

[rustfmt](https://github.com/rust-lang/rustfmt) has more information on how this works.
To put it simply:

1. Install it
    ```
    rustup component add rustfmt
    ```
1. You can run `rustfmt` on a single file:
    ```
    rustfmt src/path/to/your/file.rs
    ```
   ... or format the entire project with
   ```
   cargo fmt
   ```
   When run through `cargo` it will format all bin and lib files in the current crate.

For more context, feel free to check out the `rustfmt` project. [rustfmt](https://github.com/rust-lang/rustfmt)


#### Finding Issues with Clippy

Clippy is a code analyzer/linter to catch common mistakes and improve your Rust code!
Just like formatting your code with `rustfmt`, running clippy regularly and before your pull Request will keep our code easy to maintain.

To learn more, check out [rust-clippy](https://github.com/rust-lang/rust-clippy)

1. Install clippy:
    ```
    rustup component add clippy
    ```
2. Run clippy:
    ```
    cargo clippy
    ```

Clippy has an ever growing list of checks, that are managed in [lint files](https://rust-lang.github.io/rust-clippy/master/index.html).

### Making a pull request

When you're comfortable with your changes to be integrated into Rusty, you can create a pull request on GitHub.
We will then approve the changes or request some changes beforehand.

We encourage you to run [Clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt) on the code first.
This keeps our codebase clean.

Thanks for reading! Have fun! 🥳