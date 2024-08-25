{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs =
    with pkgs;
    [
      openssl
      rustc
      cargo
      gcc
      rustfmt
      clippy
      sqlite
      bacon
      sea-orm-cli
    ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
