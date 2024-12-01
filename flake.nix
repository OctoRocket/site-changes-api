{
    description = "Build or run the DNS record updater";

    inputs = {
        nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
        flake-utils.url = github:numtide/flake-utils;
    };

    outputs = { nixpkgs, flake-utils, ... }:

    flake-utils.lib.eachDefaultSystem (system:
    let
        pkgs = import nixpkgs { inherit system; };
    in {
        devShells.default = pkgs.mkShell rec {
            buildInputs = with pkgs; [
                # For convenience
                bacon

                # For building
                pkg-config
                openssl.dev
            ];
        };
        packages.default = pkgs.callPackage ./default.nix {};
    });
}
