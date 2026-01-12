# Proxy Tutorial

> *Thanks [Magisk Tutorial for KaniPS](https://git.fufufu.moe/KaniPS/Atrahasis/src/branch/main/Docs/Magisk-Tutorial.md)*

> [!CAUTION]  
> **This tutorial is used for Android 10 or above since it is getting harder to install root certificates on later versions of Android. This tutorial is also backward compatible with Android 10 or below.**
> 
> **Mostly used in MumuPlayer emulator.**

## Prerequisites

### Windows
- [Python 3.12 or later](https://mitmproxy.org/)
- Mitmproxy 12.x or later installed via pip

### Android (or Emulator)
- Magisk ([KitsuneMagisk](https://github.com/1q23lyc45/KitsuneMagisk)) -> *[Video Tutorial for MumuPlayer](https://youtu.be/E4VjZUMxYFE)*
  - Module: [cert-fixer](https://github.com/pwnlogs/cert-fixer)
- [WireGuard](https://www.wireguard.com/)
- Root Certificate Manager (for MumuPlayer)
- GIRLS' FRONTLINE CN

## Installation Steps

`01` Obtain the proxy script `[./docs/proxy.example.py](/docs/proxy.example.py)` from the project and rename it to `proxy.py`, then modify the `SDK_SERVER_HOST` and `SDK_SERVER_PORT` values within the file.

`02` Run `proxy.py` via `mitmweb`.
``` sh
mitmweb -m wireguard --no-http2 -s proxy.py --set termlog_verbosity=warn
```

`03` Open the mitmproxy webpage, usually located at http://127.0.0.1:8080 or 8081. If it doesn't show up, go to the webpage, navigate to the **Capture Tab**, scroll down, and find the QR code.

`04` Open the `WireGuard` app on your device, click the plus (+) sign, and scan the QR code displayed on the webpage. Activate the WireGuard connection.

`05` Open [mitm.it](http://mitm.it) in your phone/emulator browser and download the certificate.

`06` Install the certificate using `Root Certificate Manager` or via the system settings (usually provided by the emulator/Android).

`07` Restart the phone/emulator.

## Startup Steps

`01` Ensure that step `01` of the **Installation Steps** has been executed.

`02` Activate the installed WireGuard connection.