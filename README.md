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
flatpak install flathub dev.crabnebula.Platform
flatpak install flathub dev.crabnebula.Sdk
```

- Setup cargo project for flatpak. See [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/blob/master/cargo/README.md) for more details. Here are files you will need to have at leaset:
  - flatpak-cargo-generator.py
  - flatpak manifest file (ie. app.example.demo.yml)

- Build the flatpak application and run:

```
cargo b --example demo
python3 ./flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json
touch run.sh
flatpak-builder --user --install --force-clean target app.example.demo.yml
flatpak run app.example.demo
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Roadmap

Cef-rs is looking for the best approach to use CEF. On Linux, this is achived by using same flatpak runtime. So every application can share the same library. We are looking for the similar methods on macOS and Windows. Welcome to open feature requests if the feature you look for isn't listed below. But please understand that some requests might result into not planned.

### Planned

- [ ] Add Linux ARM64 target
- [ ] Add macOS ARM64 target
- [ ] Add Windows x86_64 target

### Not Planned

- Other package formats on Linux.
- Add all possible ergonomic interfaces (ie. builder types for attributes and settings.)
- Provide tools to bundle and distribute application.
