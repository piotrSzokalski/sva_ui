# sva_ui 

School project - Education software to aid in learing basics of coding in highly abstraced psudeo assembly

![image](https://github.com/piotrSzokalski/sva_ui/assets/101019797/2573681a-c07f-4196-9ba5-02a9c9df4da5)

### Building

Project use library as a git submodule, clone it with submodules

`git clone --recurse-submodules https://github.com/piotrSzokalski/sva_ui.git`

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`



