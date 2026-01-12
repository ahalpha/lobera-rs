# 代理教程

> *Thanks [Magisk Tutorial for KaniPS]()*

> [!CAUTION]  
> **本教程适用于 Android 10 或更高版本，因为在较新版本的 Android 上安装根证书变得越来越困难。本教程同样向下兼容 Android 10 或以下版本。**
> 
> **主要用于 MuMu 模拟器。**

## 前置条件

### Windows
- [Python 3.12 或更高版本](https://mitmproxy.org/)
- 通过 pip 安装的 Mitmproxy 12.x 或更高版本

### Android (或模拟器)
- Magisk ([KitsuneMagisk](https://github.com/1q23lyc45/KitsuneMagisk)) -> *[MuMu 模拟器视频教程](https://youtu.be/E4VjZUMxYFE)*
  - 模块：[cert-fixer](https://github.com/pwnlogs/cert-fixer)
- [WireGuard](https://www.wireguard.com/)
- 根证书管理器 (Root Certificate Manager，用于 MuMu 模拟器)
- 少女前线 (国服)

## 安装步骤

`01` 获取项目中的代理脚本 `[./docs/proxy.example.py](/docs/proxy.example.py)` 为 `proxy.py`，修改其中的 `SDK_SERVER_HOST` 与 `SDK_SERVER_PORT`。

`02` 通过 `mitmweb` 运行 `proxy.py`。
``` sh
mitmweb -m wireguard --no-http2 -s proxy.py --set termlog_verbosity=warn
```

`03` 打开 mitmproxy 网页界面，通常位于 http://127.0.0.1:8080 或 8081。如果页面没有自动显示，请进入网页，导航至 **Capture (捕获)** 标签页，向下滚动找到二维码。

`04` 在设备上打开 `WireGuard` 应用，点击加号 (+)，扫描网页上显示的二维码。激活 WireGuard 连接。

`05` 在手机/模拟器浏览器中打开 [mitm.it](http://mitm.it) 并下载证书。

`06` 使用 `根证书管理器` (Root Certificate Manager) 或通过系统设置（通常由模拟器/Android 系统提供）安装该证书。

`07` 重启手机/模拟器。

## 启动步骤

`01` 确保**安装步骤**中的第 `01` 步已经执行。

`02` 激活已安装的 WireGuard 连接。