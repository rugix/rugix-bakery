<p align="center">
    <img src="https://rugix.org/img/logo.svg" width="12%" alt="Rugix Logo">
</p>
<h1 align="center">
    Rugix Bakery
</h1>
<h4 align="center">
    Build custom Linux distributions in days, not months.
</h4>
<p align="center">
  <a href="https://github.com/rugix/rugix-bakery/releases"><img alt="Rugix Bakery Version Badge" src="https://img.shields.io/github/v/tag/rugix/rugix-bakery?label=version"></a>
  <a href="https://github.com/rugix/rugix-bakery/actions"><img alt="Pipeline Status Badge" src="https://img.shields.io/github/actions/workflow/status/rugix/rugix-bakery/check-and-lint.yml"></a>
</p>

> [!NOTE]
> **Support:** This repository is covered by [Tier 1: Core](https://rugix.org/support-commitment/#tier-core) of the Rugix Support Commitment.

Rugix Bakery is an open-source build system for custom, OTA-ready Linux system
images. It is developed by the [Rugix](https://rugix.org) project.

Rugix Bakery makes building a system image (almost) **as easy as writing a
Dockerfile**. It provides a structured workflow for image customization, system
variants, testing, and release artifacts.

- **Supported Distributions**: Debian, Alpine Linux, and Raspberry Pi OS.
- **OTA Updates**: Over-the-air update capabilities powered by [Rugix Ctrl](https://github.com/rugix/rugix) out of the box.
- **Container-Based Builds**: Reproducible build environment from source to image.
- **System Variants**: Support for multiple configurations including test setups.
- **Integrated Testing**: Built-in system testing framework based on VMs.
- **SBOM Generation**: Built-in SBOM generation for regulatory compliance.

Rugix Bakery includes Rugix Ctrl update support out of the box and builds the
corresponding update bundles.

[**Get started today! Build your first system and deploy an update, all in under 30 minutes!**](https://rugix.org/docs/getting-started) 🚀

[For details, check out the documentation.](https://rugix.org/docs/bakery)

## Using Rugix Ctrl with Other Build Systems

Choose Rugix Bakery when you want to build on Debian, Alpine Linux, or Raspberry
Pi OS without maintaining a full source-based distribution build. If your
product already uses Yocto, integrate Rugix Ctrl directly with the official
[open-source Yocto layers](https://github.com/rugix/meta-rugix).

## Licensing

This project is licensed under either [MIT](https://github.com/rugix/rugix-bakery/blob/main/LICENSE-MIT) or [Apache 2.0](https://github.com/rugix/rugix-bakery/blob/main/LICENSE-APACHE) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache 2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

Made with ❤️ for OSS by [Silitics](https://www.silitics.com)
