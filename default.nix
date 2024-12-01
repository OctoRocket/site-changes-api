{ pkgs, rustPlatform, ... }:

rustPlatform.buildRustPackage rec {
    pname = "site-changes-api";
    version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [ openssl.dev ];
    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;
}
