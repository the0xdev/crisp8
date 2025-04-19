# SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "CHIP-8 emulator";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in
        with pkgs; { 
          packages = {
            default = stdenv.mkDerivation rec {
              name = "crisp8";
              src = ./.;

              RUST_BACKTRACE=1;
              buildInputs = [
              ];
              nativeBuildInputs = [
                cargo
              ];

              buildPhase = ''
                cargo build -r
              '';
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/${name} $out/bin
              '';
            };
          };
        }
    );
}
