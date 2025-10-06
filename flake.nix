# SPDX-FileCopyrightText: 2025 Funkeleinhorn <git@funkeleinhorn.com>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "Rust firmware for the Akai GX 635 remote";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.05";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          pkgs.rust-bin.fromRustupToolchainFile ./firmware/rust-toolchain.toml
        );

        # Common arguments can be set here to avoid repeating them later
        # Note: changes here will rebuild all dependency crates
        commonArgs = {
          src = craneLib.cleanCargoSource ./firmware/.;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ ];
        };

        akai-gx-635 = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          }
        );
      in
      {
        checks = {
          inherit akai-gx-635;
        };

        packages.default = akai-gx-635;

        apps.default = flake-utils.lib.mkApp {
          drv = akai-gx-635;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [ reuse espflash ];
        };
      }
    );
}
