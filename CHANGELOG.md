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
