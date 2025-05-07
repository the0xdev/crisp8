# SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
#
# SPDX-License-Identifier: GPL-3.0-or-later

# https://gitlab.com/scvalex/sixty-two/-/blob/flake-blogpost/flake.nix
# is the base for this flake
{
  description = "CHIP-8 emulator";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    naersk,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk' = pkgs.callPackage naersk {};
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
        pname = "crisp8";
      in rec
        { 
          packages = {
            default = naersk'.buildPackage {
              src = ./.;
              doCheck = true;
              pname = pname;

              nativeBuildInputs = with pkgs; [
                makeWrapper
              ];
              buildInputs = with pkgs; [
                xorg.libxcb
              ];
              postInstall = ''
                wrapProgram "$out/bin/${pname}" --prefix LD_LIBRARY_PATH : "${libPath}"
              '';
            };
          };
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              cargo-insta
              pre-commit
              rust-analyzer
              rustPackages.clippy
              rustc
              rustfmt
              tokei
              xorg.libxcb
            ];
            LD_LIBRARY_PATH = libPath;
          };
        }
    );
}
