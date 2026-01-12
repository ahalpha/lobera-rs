# 快速入门

`01` 从 [发布](https://github.com/ahalpha/lobera-rs/releases) 下载预编译的可执行文件并解压到指定目录。

`02` 修改 `./gameserver.toml` 和 `sdkserver.toml` 文件：

- 游戏服务器的 `host` 值应设置为您的网络适配器的 IP 地址。
- 两个配置文件中的 `port` 值必须是未被占用的端口。
- `sdkserver.toml` 中的 `gameserver_host` 和 `gameserver_port` 值必须与 `./gameserver.toml` 中设置的 `host` 和 `port` 值一致。

`03` 运行 `./lobera-sdk-server.exe` 和 `./lobera-game-server.exe`。

`04` 现在，您可以 [配置代理](/docs/proxy_tutorial_cn.md) 以允许客户端连接到服务器。