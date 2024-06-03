{
  description = "A Nix-flake-based Rust development environment for PenTestDB";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    # esp32 = {
    #   url = "github:knarkzel/esp32";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # }; 
  };

  outputs = { self, nixpkgs, ... }:
    let
      # system should match the system you are running on
      inherit (nixpkgs);
      system = "x86_64-linux";
      # pythonPackageOverrides = self: super: {
      #   pyparsing = super.buildPythonPackage rec {
      #     pname = "pyparsing";
      #     version = "2.3.1";
      #     doCheck = false;
      #     src = super.fetchPypi {
      #       inherit pname version;
      #       sha256 = "0yk6xl885b91dmlhlsap7x78hk2rdr879fln9anbq6k4ca42djb6";
      #     };
      #   };
      # };
      #
      # idf-package-overlay = self: super: {
      #   python2 = super.python2.override {
      #     packageOverrides = pythonPackageOverrides;
      #   };
      # };
      #
      # pkgs = import (builtins.fetchTarball {
      #   # https://releases.nixos.org/nixos/unstable/nixos-20.09pre223023.fce7562cf46
      #   name = "nixos-unstable-2020-04-30";
      #   url = "https://github.com/nixos/nixpkgs/archive/fce7562cf46727fdaf801b232116bc9ce0512049.tar.gz";
      #   sha256 = "14rvi69ji61x3z88vbn17rg5vxrnw2wbnanxb7y0qzyqrj7spapx";
      # }) {
      #   overlays = [
      #     idf-package-overlay
      #   ];
      # };
      #
      # rust-esp = pkgs.callPackage (builtins.fetchTarball {
      #   name = "rust-esp-nix";
      #   url = "https://github.com/sdobz/rust-esp-nix/archive/791e35c4822a7bdb91a2fbf7323e64255b640bd0.tar.gz";
      #   sha256 = "0qp3myqpnprf7wfxxvnxpkhs3rg1d85cd9zynrhva7clgs3axnn4";
      # }) {};
    in
    {
      devShells."${system}".default =
        let
          pkgs = import nixpkgs {
            inherit system;
            # overlays = [
            #   (self: super: rec { })
            #   # cargo
            #   # rustup
            #   # gdb
            #   # openssl
            #   # pkg-config
            # ];
            # config = {
            #   permittedInsecurePackages = [
            #     "python-2.7.18.6"
            #   ];
            # };
          };
        in
        pkgs.mkShell { 
          # create an environment with nodejs-18_x, pnpm, and yarn
          packages = [
            pkgs.gdb
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rust-analyzer
            pkgs.rustfmt
            pkgs.clippy
            pkgs.rusty-man
            pkgs.esptool
            pkgs.podman
            pkgs.rustc
            pkgs.cargo
            pkgs.cargo-espmonitor
            pkgs.cargo-espflash
            pkgs.just
            # pkgs.cargo-generate
            # pkgs.espup
            # pkgs.writeShellApplication {
            #   name = "cargo";
            #   runtimeInputs = [ pkgs.ripgrep ];
            #   text = ''
            #     #!${pkgs.stdenv.shell}
            #     ./.cargo/cargo $@
            #   '';
            #   checkPhase = "${pkgs.stdenv.shellDryRun} $target";
            # }
          ];
          shellHook = ''
            IN_NIX_DEV="yes"
          ''; 
        };
    }; 
}
