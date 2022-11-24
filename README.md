# YOUTUBE-DL-REPL

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) is a frontend for [youtube-dl](https://github.com/ytdl-org/youtube-dl) providing an interactive shell mode (REPL).

# EXAMPLES

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) takes command-line arguments of [youtube-dl](https://github.com/ytdl-org/youtube-dl) and then, makes a loop:

![example-image-1](https://github.com/piotrbajdek/youtube-dl-repl/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) should run smoothly on **Windows** and **macOS**, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and primarily tested on **Fedora Linux**.

[youtube-dl-repl](https://github.com/piotrbajdek/youtube-dl-repl) (in Rust) requires [youtube-dl](https://github.com/ytdl-org/youtube-dl) (in Python) to be installed on the system as a dependency. Note that to make them work together you need to perform a separate installation of [youtube-dl](https://github.com/ytdl-org/youtube-dl) e.g., from your Linux distro's repositories.

youtube-dl-repl v1.0.0:

– Was successfully tested on Arch Linux, Fedora Linux 37, openSUSE Tumbleweed, Ubuntu 22.04 and Ubuntu 22.10.

– Failed to run on Mageia 8 due to an old glibc version (required ≥2.34).

## METHOD 1 – BY THE USE OF CARGO

**[recommended for programmers]**

**1.** Install from crates.io by the use of cargo:

_cargo install youtube-dl-repl_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy youtube-dl-repl to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (can be set up by [rustup](https://www.rust-lang.org/tools/install)).

## METHOD 2 – LINUX UNIVERSAL BINARIES

**1.** Download the distro-independent [binary](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.0/youtube-dl-repl) of youtube-dl-repl from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./youtube-dl-repl_

**3a.** On most Linux distros, install youtube-dl-repl via copying the binary to `/usr/bin/`:

_sudo cp youtube-dl-repl /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp youtube-dl-repl /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[recommended for most users]**

Distro-specific packages are also available for download for [.rpm](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.0/youtube-dl-repl-1.0.0-1.x86_64.rpm)- and [.deb](https://github.com/piotrbajdek/youtube-dl-repl/releases/download/v1.0.0/youtube-dl-repl_1.0.0_amd64.deb)-based Linux distros. Installation instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i youtube-dl-repl-1.0.0-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install youtube-dl-repl-1.0.0-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i youtube-dl-repl_1.0.0_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

Download and unpack the youtube-dl-repl [source](https://github.com/piotrbajdek/youtube-dl-repl/archive/refs/tags/v1.0.0.zip) from GitHub. Then, build and install the program:

_cargo build \--release && sudo cp target/release/youtube-dl-repl /usr/bin/_
