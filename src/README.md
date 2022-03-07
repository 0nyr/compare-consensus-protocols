# PoW - Proof of work

### Useful links

[Measure power consuption of a process](https://askubuntu.com/questions/1241160/what-is-the-best-way-to-observe-energy-usage-of-a-program-on-linux)

[std::net | rust](https://doc.rust-lang.org/std/net/index.html)

[echo server in rust](https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo)

> Make a minimal blockchain node enabling PoW

The node has to:

* connect to other nodes
* check for PoW success from other nodes
* calculate hash and try to solve the PoW
* if PoW success, write its name to the blockchain


Node

-> Try to connect to other node by sending TCP handshake to every thing available (localhost), hoping for a response.

-> If no response, the node consider it is a *genesis* node, then start building the blockchain by building a *genesis* block.


Questions

How do a node nows a public key of an address ?


## Notes

Don't hash the transactions. Just store addresses as strings.
