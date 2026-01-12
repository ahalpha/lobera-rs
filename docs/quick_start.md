# Quick Start

`01` Download the pre-compiled executables from [Releases](https://github.com/ahalpha/lobera-rs/releases) and extract them into a specific directory.

`02` Modify `./gameserver.toml` and `sdkserver.toml`:

 - The `host` value of gameserver in both configurations should be set according to your network adapter's IP address.
 - The `port` value in both configurations must not be a port that is already in use.
 - The `gameserver_host` and `gameserver_port` values in `sdkserver.toml` must correspond to the `host` and `port` values set in `./gameserver.toml`.

`03` Run `./lobera-sdk-server.exe` and `./lobera-game-server.exe`.

`04` Now, you can [configure a proxy](/docs/proxy_tutorial_en.md) to allow the client to connect to the server.