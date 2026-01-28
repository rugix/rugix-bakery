{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages =
    with pkgs;
    [
      git
      just

      cmake
      go

      cargo-deny

      podman

      pkgsCross.musl64.stdenv.cc
      pkgsCross.aarch64-multiplatform-musl.stdenv.cc
    ]
    ++ [
      inputs.sidex.packages.${stdenv.hostPlatform.system}.default
    ];

  env = {
    # x86_64-unknown-linux-musl
    CC_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-gcc";
    CXX_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-g++";
    AR_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-ar";
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = "x86_64-unknown-linux-musl-gcc";
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_STRIP = "x86_64-unknown-linux-musl-strip";
    # aarch64-unknown-linux-musl
    CC_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-gcc";
    CXX_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-g++";
    AR_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-ar";
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "aarch64-unknown-linux-musl-gcc";
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_STRIP = "aarch64-unknown-linux-musl-strip";
  };

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      version = "latest";
      targets = [
        "x86_64-unknown-linux-musl"
        "aarch64-unknown-linux-musl"
      ];
    };
    javascript = {
      enable = true;
      npm.enable = true;
      pnpm.enable = true;
    };
    nix = {
      enable = true;
    };
  };
}
