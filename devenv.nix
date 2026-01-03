{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [
    git
    just
  ] ++ [
    inputs.sidex.packages.${stdenv.hostPlatform.system}.default
  ];

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      version = "latest";
      mold.enable = true;
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
