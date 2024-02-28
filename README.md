Prostredi
---------

Ubuntu 64bit

Autori
------

Nazev tymu
- xnavram00 Martin Navrátil
- xgajdo33 Richard Gajdošík
- xcernim00 Matěj Černický
- xcapkar00 Karel Josef Čáp

Licence
-------

Tento program je poskytovan pod licencí GNU General Public License v3.0

Instalace
---------
- rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- tauri
`cargo install create-tauri-app --locked`
- knihovny
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```
`cargo install tauri-cli`
