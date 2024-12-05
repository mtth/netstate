# `netstate(1)`

Run hooks on network state change under `systemd-networkd`.


## Quickstart

`netstate` can be installed from the [AUR][]. The package includes a suitable
systemd unit file:

```sh
systemctl enable --user netstate
```

Executable files in `$XDG_DATA_HOME/netstate/hooks.d` will then be automatically
invoked on connectivity changes. For example, you can get a notification each
time with the following script:

```sh
# $XDG_DATA_HOME/netstate/hooks.d/10-notify.sh
notify-send -a netstate "Network state: $1"
```

See `man netstate` for more information.


[AUR]: https://aur.archlinux.org/packages/netstate
