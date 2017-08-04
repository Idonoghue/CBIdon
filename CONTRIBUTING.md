# Overview

* [Reporting bugs]()
* [Contributing to the Design Doc]()
* [Contributing to the code]()

# Reporting Bugs

Make sure you read: [how to report bugs](https://github.com/citybound/citybound/wiki/How-to-report-bugs)

# Contributing to the Design Doc

The [Design Doc](game/README.md) outlines the philosophy and decisions that I follow when implementing Citybound you can make suggestions from individual features to complete system design proposals.

* If you have some rough ideas, it is probably best to discuss them in the [official community](https://reddit.com/r/Citybound) first, where people can give you first feedback and point you to existing relevant [Design Proposal pull-requests](https://github.com/citybound/citybound/pulls?utf8=✓&q=is%3Apr%20label%3A%22DESIGN%20PROPOSAL%22%20)
* If you have an original and detailed proposal, start editing relevant documents of the Design Doc, or add new ones
  * [Small tutorial on how to do that in the GitHub Web interface](https://help.github.com/articles/editing-files-in-another-user-s-repository/)
  * Please give your Pull Request the DESIGN PROPOSAL label
  * Invite people from the [official community](https://reddit.com/r/Citybound) or authors of other Design Proposals to comment on and suggest improvements/clarifications to your pull request
  * I will take a look at your pull request and give it a detailed review if it meets minimal quality standards
  * We will iterate on it together, a process in which you have the opportunity to explain your motivation and potentially convice me to do things in the way you suggested
  * **Make sure to <a href="https://www.clahub.com/agreements/citybound/citybound">sign the Contributor License Agreement</a>.**
  * In the end, we either
     * fully agree with your proposal and merge it into the official design doc, or:
     * we identify a compromise of a subset of the proposed changes and merge that, or:
     * if our disagreement is too large, the pull request gets closed, but with a thorough explanation from my side

# Contributing Code

* **Make sure to <a href="https://www.clahub.com/agreements/citybound/citybound">sign the Contributor License Agreement</a>.**
* **[Have a look at the documentation](http://citybound.github.io/citybound)**

## Compiling Citybound from source 

Currently Citybound is built with Rust `nightly-2017-06-08`

Recommended setup:
* Install [rustup](https://rustup.rs/) and [git](https://git-scm.com/)
* `git clone https://github.com/citybound/citybound.git`
* `cd citybound`
* Windows:
  * `rustup override add nightly-2017-06-08-x86_64-pc-windows-msvc`
  * Install the [Visual C++ 2015 Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools), unless you have Visual Studio 2015
* MacOS:
  * `rustup override add nightly-2017-06-08-x86_64-apple-darwin`
* Linux:
  * `rustup override add nightly-2017-06-08-x86_64-unknown-linux-gnu`
* `cargo run --release` (Debug mode is generally too slow to interact with)

## Getting the recommended dev environment

* Install [Visual Studio Code](https://code.visualstudio.com)
  * It's a (cross-platform + JS-based + rich plugin ecosystem) Editor like Atom, only snappier - (it also has nothing to do with Visual Studio)
  * Yes it's actually cool, because Microsoft has started to be cool.
* Install [the VSCode-Rust Extension](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)
  * Let it install everything it wants to
  * *If you are using Windows and have a space in your user name:*
    * Create a symbolic link to you user folder that doesn't contain a space
      * for example `C:\firstname` -> `C:\Users\Firstname Lastname`
    * Add the following user settings in VSCode
      * `"rust.cargoHomePath": "C:\\firstname\\.cargo"`,
      * `"rust.racerPath": "C:\\firstname\\.cargo\\bin\\racer.exe"`,
      * `"rust.rustLangSrcPath": "C:\\firstname\\.rustup\\toolchains\\nightly-2017-06-08-x86_64-pc-windows-msvc\\lib\\rustlib\\src\\rust\\src"`
  * Otherwise it "should just work"
* For debugging (Linux/MacOS): Install the [LLDB Debugger Extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
* Now everything should just work! (fingers crossed)

## Conforming to style

* install rustfmt: `cargo install rustfmt --vers 0.9.0` **and please make sure to use the same version as noted here** (pinned now, but might change from time to time)
* run rustfmt on the whole repo:
  `rustfmt --write-mode=overwrite ./src/main.rs ./lib/*/src/lib.rs`
  (using default settings) - if there are any overlong lines it can't fix, please fix them manually.
* You should also **fix all clippy warnings** properly

## Have a question? Want to discuss something?

Join me and the other contributors in the [Gitter community for Citybound](https://gitter.im/citybound/Lobby) and ask/discuss away!

## State of the code & organization

The code is in a pretty messy state after a rushed first release, but will become much more modular and well-documented over time.
Issues are categorized into levels of difficulty amongst other properties, but the ones flagged with "Assistance Welcome" are most likely to be tackleable by outside contributors.
Pull requests of any kind are welcome, but there is no defined process or acceptance criteria yet, we'll just figure it out along the way.
