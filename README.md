Prostředí
---------

Ubuntu 64bit

Autoři
------

Hrdza
- xnavram00 Martin Navrátil
- xgajdo33 Richard Gajdošík
- xcernim00 Matěj Černický
- xcapkar00 Karel Josef Čáp

Licence
-------

Tento program je poskytován pod licencí GNU General Public License v3.0

Instalace
---------
```bash
cd calculator-main/src
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    cargo \
    make \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```
Použité virtuální prostredi
---------
Ubuntu 22.04 – 64bit
- profiling
`cargo install flamegraph --locked`