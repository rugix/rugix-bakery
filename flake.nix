{
  description = "Project development environment";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          system-tests = pkgs.mkShell {
            packages = with pkgs; [
              podman
              qemu
              mtools
              swtpm
              openssl
            ];
            RUGIX_OVMF_CODE_AMD64 = "${pkgs.OVMF.fd}/FV/OVMF_CODE.fd";
            RUGIX_OVMF_VARS_AMD64 = "${pkgs.OVMF.fd}/FV/OVMF_VARS.fd";
          };
          default = pkgs.mkShell {
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.xz
              pkgs.openssl
              pkgs.stdenv.cc.cc.lib
            ];
            CC_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-gcc";
            CXX_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-g++";
            AR_x86_64_unknown_linux_musl = "x86_64-unknown-linux-musl-ar";
            CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = "x86_64-unknown-linux-musl-gcc";
            CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_STRIP = "x86_64-unknown-linux-musl-strip";
            CC_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-gcc";
            CXX_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-g++";
            AR_aarch64_unknown_linux_musl = "aarch64-unknown-linux-musl-ar";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "aarch64-unknown-linux-musl-gcc";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_STRIP = "aarch64-unknown-linux-musl-strip";
            packages = with pkgs; [
              git
              cmake
              cargo-deny
              cargo-cyclonedx
              openssl
              xz
              pkg-config
              podman
              pkgsCross.musl64.stdenv.cc
              pkgsCross.aarch64-multiplatform-musl.stdenv.cc
            ];
          };
        }
      );
    };
}
