Rust binding for the Rime Input Method Engine.

And a server for using Rime over the network.


If you have Rime installed locally,
use `cargo run --bin ttw --features term` to try a demo locally in terminal.


You can also start a server with
`cargo run --bin rime-server-simple  --features="network"`.

On a computer without Rime,
you can connect to the server and start typing,
using `cargo run --bin rime-client-term --features="network" -- 192.168.1.1:17878`.
The last parameter should be the IP address and port of the server.
