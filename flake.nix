{
  description = "Minimal Rust Development Environment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
    andoriyu = {
      url = "github:andoriyu/flakes";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };
    devshell.url = "github:numtide/devshell/master";
  };
  outputs =
    { self, nixpkgs, rust-overlay, flake-utils, andoriyu, devshell, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        cwd = builtins.toString ./.;
        overlays = [ devshell.overlay rust-overlay.overlay andoriyu.overlay andoriyu.overlays.rust-analyzer ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile "${cwd}/rust-toolchain.toml";
      in with pkgs; {
        devShell = pkgs.devshell.mkShell {
          packages = [
            openssl
            openssl.dev
            pkgconfig
            rust
            andoriyu-ra.rust-analyzer.latest
            cargo-expand-nightly
            ruby_3_0
          ];
          bash = {
            extra = ''
              export LD_INCLUDE_PATH="$DEVSHELL_DIR/include"
              export LD_LIB_PATH="$DEVSHELL_DIR/lib"
            '';
            interactive = "";
          };
          env = [
            {
              name = "RUST_SRC_PATH";
              value = "${rust}/lib/rustlib/src/rust/library";
            }
            {
              name = "OPENSSL_DIR";
              value = "${openssl.bin}/bin";
            }

            {
              name = "OPENSSL_LIB_DIR";
              value = "${openssl.out}/lib";
            }

            {
              name = "OPENSSL_INCLUDE_DIR";
              value = "${openssl.out.dev}/include";
            }
          ];
        };
      });
}


