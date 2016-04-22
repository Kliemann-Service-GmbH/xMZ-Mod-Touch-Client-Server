# Vorbereitungen



## Start Server

    cargo run --bin server

## Starting the client

    cargo run --bin client

# systemd unit
Zu Begin wird ein Unit File erstellt

    vim /etc/systemd/system/xmz-server.service

Folgender Inhalt ist nötig

    [Unit]
    Description="LED und RELASI Control"
    After=multi-user.target

    [Service]
    ExecStart=/root/server &

    [Install]
    WantedBy=multi-user.target

Danach muss der service noch aktiviert ...

    systemctl enable xmz-server.service

Und gestartet werden.

    systemctl restart xmz-server.service

**Wichtig ist zu erwähnen dass man nich `Type=oneshot, forking` oder so angeben darf. Wird Type weg gelassen dann ist der Type simple.
Das bedeutet der Deamon kann nicht selbst in den Background gehen, das ist warscheinlich wegen der primitiven Thread handhabung in meiner Lösung so.**


# Links
* http://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/sockets.html
* Command Line Parser https://github.com/kbknapp/clap-rs
