# Motoko and Candid Language Support for Zed

[![Zed Extension][zed-extension-badge]][zed-extension-url]
[![License][license-badge]][license-url]

[zed-extension-badge]: https://img.shields.io/badge/Zed%20Extension-%230951CF?style=flat&logo=zedindustries&logoColor=white&labelColor=black
[zed-extension-url]: https://zed.dev/extensions/motoko
[license-badge]: https://img.shields.io/badge/License-Apache_2.0_OR_MIT-blue.svg?style=flat&labelColor=black&color=blue
[license-url]: #license

<p align="center">
  <img src="https://github.com/perforate-org/zed-motoko/raw/main/assets/zed_motoko_icon_256.avif" alt="Zed Motoko Icon" />
</p>

Motoko and Candid language support for [Zed](https://zed.dev/), allowing you to develop applications easily for [Internet Computer](https://internetcomputer.org/).

- Tree-sitter for Motoko: [christoph-dfinity/tree-sitter-motoko](https://github.com/christoph-dfinity/tree-sitter-motoko)
- Tree-sitter for Candid: [perforate-org/tree-sitter-candid](https://github.com/perforate-org/tree-sitter-candid)
- Language Server for Motoko: A part of [dfinity/vscode-motoko](https://github.com/dfinity/vscode-motoko)

## Prerequisites

For certain tasks:

- [IC SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
- [Mops](https://docs.mops.one/quick-start)
- [mo-dev](https://github.com/dfinity/motoko-dev-server#readme)

## Configuring

### Formatter

The Motoko formatter is based on Prettier, and you can configure it for each project using `.prettierrc`.
For detailed configuration instructions, please refer to [dfinity/prettier-plugin-motoko](https://github.com/dfinity/prettier-plugin-motoko?tab=readme-ov-file#customization).

Additionally, you can disable the formatter using Zed's settings:

```jsonc
{
  "lsp": {
    "motoko-language-server": {
      "initialization_options": {
        "formatter": "none" // "prettier" by default
      }
    }
  }
}
```

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

To contribute to this extension:

1. Clone the repository
2. Make your changes
3. Test with Zed
4. Submit a pull request

## License

This project is licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT License](./LICENSE-MIT) at your option.
