{
  description = "Rust broker-v2 dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixkgs";
    };
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    crate2nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        cargoNix = crate2nix.tools.${system}.appliedCargoNix {
            name = "gui_panic_handler";
            src = ./.;
        };

        # TODO: Is anything superflous here?
        eguiLibs = with pkgs; [
          wayland
          libxkbcommon
          libGL
          libGLU
        ] ++ (with pkgs.xorg; [
          libX11
          libxcb
          libXcursor
          libXrandr
          libXi
          pkg-config
        ]);
      in {
        packages.default = pkgs.symlinkJoin {
          name = "gui_panic_handler_test";
          paths = [ cargoNix.rootCrate.build ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/gui_panic_handler\
              --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath eguiLibs}
            mv $out/bin/gui_panic_handler $out/bin/gui_panic_handler_test
          '';
        };
        #;

        devShell = pkgs.mkShell rec {
          nativeBuildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" "cargo" "rustc" ];
            })
            pkgs.gcc
          ] ++ eguiLibs;

          shellHook = ''
              export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${pkgs.lib.makeLibraryPath eguiLibs}
          '';

          RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
          }}/lib/rustlib/src/rust/library";


          buildInputs = with pkgs; [
            openssl.dev
            glib.dev
            pkg-config

            clippy
            rust-analyzer
            just
          ];
        };
      }
    );
}
