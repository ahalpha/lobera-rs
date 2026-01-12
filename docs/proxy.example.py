from mitmproxy import http

SDK_SERVER_HOST = "192.168.1.2"
SDK_SERVER_PORT = 9000

REWRITE_HOST_LIST = [
    "event-sdk.megagamelog.com",
    "account-sdk.megagamelog.com",
    "sdk.megagamelog.com",
    "ext-sdk.megagamelog.com",
    "cdn.megagamelog.com",
    "cdn2.megagamelog.com",
    "notice.megagamelog.com",
]

KILL_HOST_LIST = [
    "android.bugly.qq.com",
]


def request(flow: http.HTTPFlow) -> None:
    if any(flow.request.pretty_host.endswith(host) for host in KILL_HOST_LIST):
        flow.kill()
        return
    if flow.request.pretty_host in REWRITE_HOST_LIST:
        flow.request.path = f"/{flow.request.pretty_host}{flow.request.path}"
        flow.request.scheme = "http"
        flow.request.host = SDK_SERVER_HOST
        flow.request.port = SDK_SERVER_PORT
        return


print(f"Proxying to {SDK_SERVER_HOST}:{SDK_SERVER_PORT}")
