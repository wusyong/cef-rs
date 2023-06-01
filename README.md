# cef-rs

Use CEF in Rust.

## Supported Targets

| Target | Linux | macOS | Windows |
| ------ | ----- | ----- | ------- |
| x86_64 | ✅    | ❎     | ❎      |
| ARM64  | ❎    | ❎     | ❎      |

## Usage

### Linux

- Install flatpak runtime & sdk:

```
TODO
```

- Setup cargo project for flatpak. See [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/blob/master/cargo/README.md) for more details. Here are files you will need to have at leaset:
  - flatpak-cargo-generator.py
  - flatpak manifest file (ie. app.tauri.demo)

- Build the flatpak application and run:

```
cargo b --example demo
python3 ./flatpak-cargo-generator.py ./quickstart/Cargo.lock -o cargo-sources.json
touch run.sh
flatpak-builder --user --install --force-clean target app.tauri.demo.yml
flatpak run app.tauri.demo
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Roadmap

Welcome to open feature requests if the feature you look for isn't listed below. But please understand that some requests might result into not planned.

### Planned

- Find out the best approach to use CEF in the target we support.
- [ ] Add Linux ARM64 target
- [ ] Add macOS ARM64 target
- [ ] Add Windows x86_64 target

### Not Planned

- Other package formats on Linux.
- Add all possible ergonomic interfaces (ie. builder types for attributes and settings.)
- Provide tools to bundle and distribute application.
