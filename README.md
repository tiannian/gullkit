# Gullkit

A ultimate network and protocol toolkit based on dynamic graph.

## Usage

### Daemon

Run on terminal directly.

```shell
$ gullkitd
```

Or as systemd service

```bash
# systemctl enable --now gullkitd.service
```

## Controller Command

### Expose a path

```bash
$ gullkit path httpserve .
```

## Configure

## Controller API

## Support Protocol

### Input

- Http(s)
- Websocket
- TUN (udptcp mode)
- DHCP
- DNS
- Http proxy
- Socks5
- Trojan
- Jsonrpc

### Output

- Http(s)
- Filesystem
- Websocket
- DHCP
- DNS
- Http proxy
- Socks5
- Trojan
- uwsgi
- Dbus
- Jsonrpc

