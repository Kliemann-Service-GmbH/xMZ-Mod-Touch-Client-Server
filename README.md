# Vorbereitungen (development und production)
Folgende Software muss auf der xMZ-Mod-Touch Hardware vorab installiert werden.
```
apt-get install libnanomsg-dev
```

# Quellcode auschecken
Zunächst wird das Super Repository, welches alle Software Komponenten vereint,
auf der xMZ-Mod-Touch Hardware ausgecheckt. Siehe dazu auch dessen README.md
https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software

```
cd
git clone https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software.git
cd xMZ-Mod-Touch-Software
git submodule init
git submodule update
```

# Kompellierung
## Kompellierung und Deployment auf die xMZ Hardware (production)
```
cd
cd xMZ-Mod-Touch-Software
cd xMZ-Mod-Touch-Client-Server
```

Dann kann die Software kompelliert ...
```
cargo build --target=armv7-unknown-linux-gnueabihf --bin server --release
cargo build --target=armv7-unknown-linux-gnueabihf --bin client --release
```

... und installiert werden.
```
cp -v target/armv7-unknown-linux-gnueabihf/release/{client,server} /usr/bin/
```


## Kompellierung in der Entwicklungsumgebung (development)
```
cd xMZ-Mod-Touch-Client-Server
export xmz_mod_touch_ip=192.168.200.154
cargo build --target=armv7-unknown-linux-gnueabihf --bin server --release
cargo build --target=armv7-unknown-linux-gnueabihf --bin client --release
ssh -i ~/development/custom_image/id_rsa root@${xmz_mod_touch_ip} systemctl stop xmz-mod-touch-server.service
scp -i ~/development/custom_image/id_rsa target/armv7-unknown-linux-gnueabihf/release/{client,server} root@${xmz_mod_touch_ip}:/root/
ssh -i ~/development/custom_image/id_rsa root@${xmz_mod_touch_ip} systemctl start xmz-mod-touch-server.service
```

# Systemd Unit
Zu Begin wird ein Systemd Unit File erstellt.
```
cat <<EOF >/etc/systemd/system/xmz-mod-touch-server.service
[Unit]
Description="LED und RELASI Control"
After=multi-user.target

[Service]
ExecStart=/usr/bin/server &

[Install]
WantedBy=multi-user.target
EOF
```
**Wichtig ist zu erwähnen dass man nich `Type=oneshot, forking` oder so
angeben darf. Wird Type weg gelassen dann ist der Type simple.
Das bedeutet der Deamon kann nicht selbst in den Background gehen,
das ist warscheinlich wegen der primitiven Thread Handhabung in meiner Lösung so.**

Danach muss der service noch aktiviert ...
```
systemctl enable xmz-mod-touch-server.service
# systemctl daemon-reload # Dieser Befehl ist nur bei Änderungen am Unit File nötig!
```

... und gestartet werden.
```
systemctl restart xmz-mod-touch-server.service
```



# Links
* http://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
* Command Line Parser https://github.com/kbknapp/clap-rs
