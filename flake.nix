{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            sqlitebrowser
          ];
          nativeBuildInputs = with pkgs; [
            bacon
            pkg-config
            gobject-introspection
            cargo
            cargo-update
            cargo-tauri
            nodejs
            bun
            xsel
            uv
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            sqlx-cli
            webkitgtk_4_1
            openssl
          ];

          shellHook = ''
            export RUSTUP_TOOLCHAIN=stable
          '';
        };

      }
    );
}
