# Change log history

### Useful links

[What is a Changelog.md file?](https://changelog.md/)

## History

### Wed 2 March 2022

Trying to understand what to do. Created the repository and initial LaTeX code. Got *Bitcoin white paper*, and *Blockchain Consensus: An Overview of Alternative Protocols*.

### TODOs

* [ ] Understand how to make a sheme with Inkscape and LaTeX.
* [ ] Read *Bitcoin white paper* Proof-of-work section and make a summary.
* [ ] Read *Blockchain Consensus: An Overview of Alternative Protocols*
* [ ] Read [blockchain | wikipedia](https://en.wikipedia.org/wiki/Blockchain)

### Wkd 5-6 March 2022

Read the Wikipedia [blockchain](https://en.wikipedia.org/wiki/Blockchain) article to get me started with the basics.

### Mon 7 March 2022

Started to code a blockchain node from scratch. I'm still having many questions on what features I should have and what to keep minimal. My goal is to have nodes capable of connecting to one another, and mining blocks.

* [ ] Finish implementing Blockchain
* [ ] Making a CLI to start the node on a specified port
* [ ] scanning ports
* [ ] establishing inter-node connection.
* [ ] Trying the node alone first.

### Wed 9 March 2022

Added more code to the simple blockchain project in Rust.

##### VSCode intellisense not working

Trying to continue to build the node.

Having issues with VSCode code intellisense.

```shell
> Executing task: rustup component add rust-analysis --toolchain nightly-x86_64-unknown-linux-gnu <

info: component 'rust-analysis' for target 'x86_64-unknown-linux-gnu' is up to date

Terminal will be reused by tasks, press any key to close it.

> Executing task: rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu <

info: component 'rust-src' is up to date

Terminal will be reused by tasks, press any key to close it.

> Executing task: rustup component add rls --toolchain nightly-x86_64-unknown-linux-gnu <

error: component 'rls' for target 'x86_64-unknown-linux-gnu' is unavailable for download for channel 'nightly'
Sometimes not all components are available in any given nightly. If you don't need the component, you can remove it with:

    rustup component remove --toolchain nightly --target x86_64-unknown-linux-gnu rls
The terminal process "/usr/bin/bash '-c', 'rustup component add rls --toolchain nightly-x86_64-unknown-linux-gnu'" terminated with exit code: 1.

Terminal will be reused by tasks, press any key to close it.
```

Problem fixed thanks to [this | Reddit](https://www.reddit.com/r/rust/comments/7umj04/nightly_or_stable_you_may_have_no_choice/), and [this | GitHub issue](https://github.com/rust-lang/vscode-rust/issues/237#issuecomment-359639894) ⭐️.

### Thu 10 March 2022

Trying to understand Rust scopes, borrowing and concurrency. This is very mandatory to be able to build the node, since we need to have several threads for the web client and the miner, in order to get transactions, add them to the block, mine the block through PoW then sending it to the peers.

* [X] Read [this guide on concurrency in Rust](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html).
* [ ] Read [this blog post of Rust scoping system](https://blog.skylight.io/rust-means-never-having-to-close-a-socket/).
* [X] Make a simple example of concurrency in Rust.

I made a simple program that define a `struct` with a vector and a counter. The idea is to have several threads trying concurrently to update the counter after some random computations, much similar to what happens in Proof of Work when several nodes try to update the same data structure (blockchain) after some random problem solving.

The program make great use of `scope` in Rust, as well as 2 crucial components of the STD: `std::Mutex` for protecting data updates between threads using a lock, and `std::sync::Arc` to provide thread-safe smart pointer

```shell
(base) onyr@aezyr:~/Documents/code/rust/concurrency/concurrent_vect_use$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/concurrent_vect_use`
Launching the program...
initial counter: Counter { last_value: 100, updating_threads: [] }
Counter { last_value: 400, updating_threads: [0, 0, 0, 0, 1, 1, 1, 2, 0, 2, 1, 2, 2, 1, 2, 0, 1, 2, 0, 0, 2, 1, 2, 1, 2, 2, 0, 0, 1, 1] }
```

It took me several hours to read the docs and understand how to deal with concurrency in Rust. However, once the compiler is kind enough to accept the code, things just works. This is an important step in building a basic blockchain node.
