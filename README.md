# YOUTUBE-DL-REPL

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) is a user-friendly interface for [youtube-dl](https://github.com/ytdl-org/youtube-dl) that features an interactive command-line interface (REPL).

# EXAMPLES

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) utilises the command-line arguments of [youtube-dl](https://github.com/ytdl-org/youtube-dl), and subsequently enters a loop, enabling the user to continually input URLs.

![example-image-1](https://github.com/piotrbajdek/youtube-dl-repl/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) is designed to be compatible with **Windows** and **macOS**, and can be easily installed using [cargo](https://www.rust-lang.org/tools/install). However, the primary development and testing environment for youtube-dl-repl is **Fedora Linux**.

The Rust-based program [youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) requires the Python-based program [youtube-dl](https://github.com/ytdl-org/youtube-dl) to be installed on the system as a dependency. To use them together, you must separately install [youtube-dl](https://github.com/ytdl-org/youtube-dl) through your Linux distribution's package repositories.

The current version of youtube-dl-repl (v1.0.1) has been verified to work properly on Fedora Linux 37 and Ubuntu 22.10.

## METHOD 1 – USING CARGO

**[Recommended for programmers]**

**1.** To install youtube-dl-repl from [crates.io](https://crates.io/crates/youtube-dl-repl), use the following cargo command:

_cargo install youtube-dl-repl_

The executable will be saved in the hidden `.cargo/bin/` directory within your home directory.

**2a.** For easy access, you may want to copy the youtube-dl-repl file to the `/usr/bin/` directory. This can be done by following the instructions in Method 2 (3a, 3b).

**2b.** As an alternative, you can add the `~/.cargo/bin/` directory to your system's PATH variable, which can be configured using [rustup](https://www.rust-lang.org/tools/install).

## METHOD 2 – UNIVERSAL LINUX BINARIES

**1.** To install youtube-dl-repl, first download the distro-independent [binary](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.1/youtube-dl-repl) from GitHub.

**2.** Then, make the file executable by running the command:

_sudo chmod +x ./youtube-dl-repl_

**3a.** On most Linux distributions, install youtube-dl-repl by copying the binary to `/usr/bin/`:

_sudo cp youtube-dl-repl /usr/bin/_

**3b.** For Fedora Silverblue / Kinoite, use this command:

_sudo cp youtube-dl-repl /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[Recommended for most users]**

Distro-specific packages for [.rpm](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.1/youtube-dl-repl-1.0.1-1.x86_64.rpm) and [.deb](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.1/youtube-dl-repl_1.0.1_amd64.deb)-based Linux distributions are also available for download. To install youtube-dl-repl on different Linux distributions, follow these instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i youtube-dl-repl-1.0.1-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install youtube-dl-repl-1.0.1-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i youtube-dl-repl_1.0.1_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

First, download and unpack the youtube-dl-repl [source code](https://github.com/piotrbajdek/youtube-dl-repl/archive/refs/tags/v1.0.1.zip) from GitHub. Next, to build and install the program, use the command:

_cargo build \--release && sudo cp target/release/youtube-dl-repl /usr/bin/_
