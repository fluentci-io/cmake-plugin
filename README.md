# CMake Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/cmake)](https://pkg.fluentci.io/cmake)
[![ci](https://github.com/fluentci-io/cmake-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/cmake-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [CMake](https://cmake.org/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm cmake setup
```

## Functions

| Name     | Description                               |
| -------- | ----------------------------------------- |
| setup    | Installs a specific version of cmake.     |
| generate | Generates build system files.             |
| make     | Builds the project.                       |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.3"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/cmake@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Generate Makefiles
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: cmake
    args: |
      generate
      make
      make test
  env:
    GITHUB_ACCESS_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```
