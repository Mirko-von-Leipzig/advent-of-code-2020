# Advent of Code 2020

Learning Rust by solving [Advent of Code 2020](https://adventofcode.com/2020)

## Table of Contents

- [Participating](#participating)
- [Getting setup](#getting-setup)
  - [VSCode Extensions](#vscode-extensions)
  - [Tools](#tools)
- [Resources](#resources)
- [Code Organisation](#code-organisation)
- [Continuous Integration](#continuous-integration)
- [Progress](#progress)

## Participating

1. Register with [Advent of Code 2020](https://adventofcode.com/2020) using your Github account
2. Raise an issue letting me know you want to join, I'll have to add you to the project
3. Create a pull request adding in your Rust package

    1. clone this repository
    2. navigate into the root of the repository
    3. `cargo new <name>`
    4. add `<name>` to the members of the workspace in `<root>/Cargo.toml`
    5. create a pull-request with these changes

4. Once the pull-request has been accepted you will have access
5. Feel free to raise any questions as issues

## Getting setup

We will be using the Rust programming language and VSCode as our editor.

You can obtain Rust from [here](https://www.rust-lang.org/learn/get-started) and VSCode [here](https://code.visualstudio.com/).

### VSCode Extensions

You should install the following extensions in VSCode:

| Extension | Description |
| :-------- | :---------- |
| [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) | IDE-like Rust integration in VSCode |
| [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) | Dependency helper |
| [better-toml](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) | TOML language support (used to specify dependencies) |
| [test-explorer](https://marketplace.visualstudio.com/items?itemName=hbenl.vscode-test-explorer) | Provides handy side-panel test runner |
| [rust-text-explorer](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter) | Integrates Rust tests into test-explorer |
| [error-lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) | Shows errors in-line |
| [git-lens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens) | Enhances git, and provides github integration |

Please do **NOT** install the RLS extension if prompted -- this is the old original Rust extension that is vastly exceeded by rust-analyzer. Remove it if you do have it for some reason.

### Tools

Rust comes with tools that are expected of a modern language, which are used universally in the ecosystem. These may come pre-installed, but just in-case please add the Rust formatter (`rustfmt`) and linter (`clippy`) by executing

1. `rustup component add rustfmt`
2. `rustup component add clippy`

In addition, the `rust-analyzer` extension may need to be configured to invoke the linter on save. Find the setting

```
Rust-analyzer › Check On Save: Command
Cargo command to use for cargo check
```

and change it to `clippy`.

## Resources

| Resource  | Description |
| :-------- | :---------- |
| [The Official Rust Book](https://doc.rust-lang.org/stable/book/) | Wonderfully explains the Rust language |
| [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)| Learn by examples |
| [Rust Std Library Documentation](https://doc.rust-lang.org/std/index.html) | Standard library API reference |
| [docs.rs](https://docs.rs/) | Library documentation |
| [lib.rs](https://lib.rs/) | Search engine for libraries |

## Code Organisation

We will use a Rust workspace to combine each participants code into one big package. This will allow us to selectively share code, and still keep the rest separate. A workspace effectively acts as the root of a multi-package setup. The root `Cargo.toml` specifies the members of the workspace -- you can see the `mirko` member (which links the Rust package in `mirko` to the workspace). The `mirko` folder contains its own Rust package as defined in the `mirko/Cargo.toml` file.

There are a few ways of specifying binary executable targets that you want Rust to build. I would recommend using the simplest mechanism for this -- Rust automatically identifies each toplevel file / folder in the `src/bin` directory as its own binary. In addition I will place each day's input file into the a single folder, and have at least one library containing common code shared between my solutions.

i.e. this will be my file structure:

```
src
├── bin
│   ├── day_01.rs
│   ├── day_02.rs
│   ├── day_03.rs
│   ├── ....
│   └── day_25.rs
├── lib
│   └── common_code.rs
├── inputs
│   ├── input_01.rs
│   ├── input_02.rs
│   ├── input_03.rs
│   ├── ....
│   └── input_25.rs
└── cargo.toml
```

## Continuous Integration

We have a CI workflow which runs on every push and pull-request. The `main` branch is protected by this workflow.

It includes the following checks:

- linting
- formatting
- testing

## Progress

| Day |     Mirko    |     Malan    |     Evert    |
| :-: | :----------: | :----------: | :----------: |
| 01  | :star::star: | :star::star: |              |
| 02  | :star::star: | :star::star: |              |
| 03  | :star::star: |              |              |
| 04  | :star::star: |              |              |
| 05  | :star::star: |              |              |
| 06  | :star::star: |              |              |
| 07  |              |              |              |
| 08  |              |              |              |
