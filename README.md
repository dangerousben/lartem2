# Quickstart

## Install rustc and cargo

[rustup](https://rustup.rs/) seems to be the preferred approach.  Don't blame me.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install notcurses

I did this from source:

```sh
git clone https://github.com/dankamongmen/notcurses.git
cd notcurses
mkdir build
cd build
cmake ..
```

(swear a bit then install the dependencies I should have mentioned up front)

```sh
make -j4
```

Or more if you have m4d c0rez.  Obtain root by means fair or foul and then:

```
make install
```

## Build and run lartem

```sh
cargo run
```

Will take a while the first time.  In fact it'll probably download half the internet, crunch CPU for an hour, then blurt out a meaningless error.  Have fun!
