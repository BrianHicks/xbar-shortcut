{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nmattia/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        darwinInputs =
          if pkgs.stdenv.isDarwin then
            [
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ]
          else
            [ ];
      in
      rec {
        formatter = pkgs.nixpkgs-fmt;

        # `nix build`
        packages.xbar-shortcut = naersk-lib.buildPackage {
          root = ./.;
          buildInputs = [
            pkgs.libiconv
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rustPackages.clippy
          ] ++ darwinInputs;

          doCheck = true;
          checkPhase = ''
            cargo clippy -- --deny warnings
          '';
        };
        defaultPackage = packages.xbar-shortcut;
        overlay = final: prev: { xbar-shortcut = packages.xbar-shortcut; };

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs =
            [
              pkgs.cargo
              pkgs.cargo-edit
              pkgs.rust-analyzer
              pkgs.clippy
              pkgs.rustc
              pkgs.rustfmt
              pkgs.libiconv

              # for some reason this seems to be required, especially on macOS
              pkgs.clippy
            ] ++ darwinInputs;
        };
      });
}
