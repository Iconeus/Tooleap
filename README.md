# Tooleap

Tools for Tuleap.
For instance Tuleap functions for automatic updates of artifacts.

---

## Table of Contents

- [Environment Setup](#environment-setup)
- [Installation](#installation)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)

---

## Environment Setup

### Prerequisites
- **Operating System:** Linux or WSL (already tested on OpenSUSE16)

### Dependencies
Required libraries, SDKs, or tools:
- NIX (package manager for Unix like systems)

#### Installation of NIX
From a terminal, as normal user, do
```bash
sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

### Environment Variables
To have NIX available, you need to source the profile file. Beware, the NIX environment will only be available in the terminal in which you perform the source command.

```bash
source ~/.nix-profile/etc/profile.d/nix.sh
```

If you want to avoid having to source the file everytime, then add the source of the profile file to the bashrc

```bash
vim ~/.bashrc
```

And add the following line at the end.
```bash
source ~/.nix-profile/etc/profile.d/nix.sh
```

---

## Installation

### For the compute-risk function
1. Clone the repository:
```bash
git clone [repository-url]
cd artifact-post-action/compute_risk
```

Launch a NIX shell with necessary packages
```bash
nix-shell --pure shell.nix
```

Build the function
```bash
cargo build --target wasm32-wasip1 --release
```

### Files explanation
The filesystem of the compute_risk is the following

compute_risk/
    - src/main.rs
    - Cargo.lock
    - Cargo.toml
    - Makefile
    - shell.nix

The `main.rs` file contains the function source code in Rust, supported by WASI Preview 1 (WebAssembly). This is where you will edit the behavior of your artefacts.
The `shell.nix` specifies the necessary packages to install in the NIX environment to build, convert files in WebAssembly. For instance `cargo` and `make`.

---
## Usage

Upload the binary result file `(target/wasm32-wasip1/release/post-action-compute-risk.wasm)` to your Tracker administration (Administration > Workflow > Tuleap Functions).

---
## Troubleshooting

### Certificate error
When downloading NIX or its packages listed in the shell.nix you can encounter an error indicating that certificates could not be found.
Example of message: "Problem with the SSL CA cert"

First check if SSL certificates are installed
```bash
sudo zypper install ca-certificates ca-certificates-mozilla
```

If already installed, check if the following file exists _/etc/ssl/certs/ca-certificates.crt_
If not then create it with a concatenation of all the certificate files in the _/etc/ssl/certs/_ folder.
```bash
sudo cat /etc/ssl/certs/*.pem > /etc/ssl/certs/ca-certificates.crt
```
Then retry your NIX command

### Incompatibility between NIX and bash
When running the shell.nix file, if you find an error "bash: symbol lookup error: bash: undefined symbol: rl_completion_rewrite_hook" that means that there is a conflict between absh and the readline library of your system and the one required by NIX.

Ensure that you run the nix-shell with the _pure_ parameter to avoid using your system bash environment.
```bash
nix-shell --pure
```