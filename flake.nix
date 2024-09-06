
{
  description = "Rust project with required dependencies and automatic cargo run";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
            pkgs.nodejs
            pkgs.cargo-tauri
            pkgs.pkg-config
            pkgs.cargo-tauri
            pkgs.webkitgtk
            pkgs.libsoup
            pkgs.git
            pkgs.openssl
            pkgs.glib-networking
            pkgs.python3
            pkgs.wl-clipboard
            
          ];

          shellHook = ''
            rustup toolchain install stable
            rustup default stable
            export GIO_MODULE_DIR=/nix/store/j9wkqd90c3kd7xrwyqg1imfj20l62k65-glib-networking-2.80.0/lib/gio/modules/
            
          '';
        };
      });
}