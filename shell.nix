{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  PGDATA = "${toString ./.}/.pg";
  nativeBuildInputs =
    with pkgs;
    [
      openssl
      rustc
      cargo
      gcc
      rustfmt
      clippy
      bacon
      sea-orm-cli
    ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  shellHook = ''
   export RUST_LOG="debug"
   cargo install diesel_cli --no-default-features --features postgres
'';
}
