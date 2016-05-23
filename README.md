 # Vorbereitungen (development und production)
```
apt-get install libnanomsg-dev
```

# Kompellierung und Deployment auf die xMZ Hardware (production)
```
git clone https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Client-Server.git
cd xMZ-Mod-Touch-Client-Server
```


# Kompellierung in der Entwicklungsumgebung (development)
```
export xmz_mod_touch_ip=192.168.200.154
cargo build --target=armv7-unknown-linux-gnueabihf --bin server --release
cargo build --target=armv7-unknown-linux-gnueabihf --bin client --release
ssh -i ~/development/custom_image/id_rsa root@${xmz_mod_touch_ip} systemctl stop xmz-mod-touch-server.service
scp -i ~/development/custom_image/id_rsa target/armv7-unknown-linux-gnueabihf/release/{client,server} root@${xmz_mod_touch_ip}:/root/
ssh -i ~/development/custom_image/id_rsa root@${xmz_mod_touch_ip} systemctl start xmz-mod-touch-server.service
```

```
cargo run --bin server
```

## Starting the client

```
cargo run --bin client
```

# systemd unit
Zu Begin wird ein Unit File erstellt

```
vim /etc/systemd/system/xmz-mod-touch-server.service
```

Folgender Inhalt ist nötig

```
[Unit]
Description="LED und RELASI Control"
After=multi-user.target

[Service]
ExecStart=/root/server &

[Install]
WantedBy=multi-user.target
```

Danach muss der service noch aktiviert ...

```
systemctl enable xmz-mod-touch-server.service
```

Und gestartet werden.

```
systemctl restart xmz-mod-touch-server.service
```

**Wichtig ist zu erwähnen dass man nich `Type=oneshot, forking` oder so angeben darf. Wird Type weg gelassen dann ist der Type simple.
Das bedeutet der Deamon kann nicht selbst in den Background gehen, das ist warscheinlich wegen der primitiven Thread handhabung in meiner Lösung so.**


# Links
* http://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
* Command Line Parser https://github.com/kbknapp/clap-rs
