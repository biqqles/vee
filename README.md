# :swan: vee
**vee** is a prototype for a simple model of service discovery implemented atop
[nanomsg](https://nanomsg.org). The name is a reference to
the [V formation](https://en.wikipedia.org/wiki/V_formation) that swans, geese and other
migratory birds are often seen flying in.

## Explanation
### Transceiver
The crate implements [`Transceiver`](src/protocol/transceiver.rs), a simple abstraction around a
nanomsg `Socket`. When a transceiver is instantiated, it first tries to `bind` to the address
it is given. If this fails, then it assumes that another transceiver is already bound to that
address, and connects instead.

### Nodes
The network consists of [`Nodes`](src/node.rs) which wish to communicate. Nodes have a
`Transceiver` instance which connects them to the *formation*, the address of which is
foreknown by all nodes. Traffic flowing in the formation controls all service discovery.

#### Broker
The node that uniquely binds to the formation's address is termed the *broker*. It receives
all messages sent in the formation, and all messages it sends are received by all other
nodes. This behaviour is a natural result of the choice of nanomsg's
[Bus pattern](https://nanomsg.org/gettingstarted/bus.html) for the formation.

If the broker dies, then another node can be commanded to come online, and if it binds, it
becomes the broker.

These two properties of the broker are the inspiration behind this repository's name, as it
can be thought of as the 'tip' or 'head' of the V formation.

#### Services
Nodes other than the broker (i.e. which `connect` to the formation) are termed *services*.
Periodically all services send `Hail` messages. These have as their payload the name and
address of the originating service.

The broker uses the information received from these hails to build up a record of the names
and addresses of all services in the network.

When a node wishes to directly communicate with another node, it sends a `Pair` message.
This message includes the originator's name and the name of the destination with which it
intends to communicate. The broker replies with a `Link` message that includes the address
on which communication is to take place (which is the address of the originator from the
perspective of the broker). The two services can now open a
[Pair channel](https://nanomsg.org/gettingstarted/pair.html) to communicate directly.

### Messages
There are only five [`Message`](src/protocol/message.rs) types defined:

| Name |     Direction    |                    Used for                    |
|------|------------------|------------------------------------------------|
|`Hail`|service 🡒 broker  |Identifying service to broker                   |
|`Pair`|service 🡒 broker  |Request to open channel with named destination  |
|`Link`|broker 🡒 services |Response to `Pair`                              |
|`Fail`|broker 🡒 services |Address lookup failed, or other error           |
|`Data`|service 🡒 service |Application data transfer in link mode          |


## Usage
Clone this repo, then open a few terminal sessions. Run `demo/1`, `demo/2` and
`demo/3` in each of these to see a quick demo. Run `demo/end` to end the demo.

`cargo run` will give you a usage hint for starting your own nodes.
