# Chain of Custody - Holochain Dev Workshop

This repository is designed to be coded following [this guide](https://holochain-gym.github.io/workshops/workshop-1).

To change the code, you can work either opening VSCode inside the root folder of the repo or in this folder, you should have rust intellisense either way. It is recommended to use the `rust-analyzer` VSCode extension.

## Requirements

- Having [`nix-shell` installed](https://developer.holochain.org/docs/install/).
- Have [`holochain-run-dna`](https://www.npmjs.com/package/@holochain-open-dev/holochain-run-dna) installed globally, and the `lair-keystore` described in its README as well.

## Enter the nix-shell

**Run this before running any command in this readme**:

```bash
$(nix-build https://holochain.love --no-link -A pkgs.holonix)/bin/holonix
```

This will setup your environment to be able to execute holochain.

## Building

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
dna-util -c todo_rename_zome.dna.workdir/
```

## Testing

After having built the DNA:

```bash
cd test
npm install
npm test
```

## Running

After having built the DNA:

```bash
holochain-run-dna todo_rename_zome.dna.gz
```

Now `holochain` will be listening at port `8888`;

Restart the command if it fails (flaky holochain start).
