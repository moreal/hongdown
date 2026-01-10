{
  description = "Hongdown flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rust = pkgs.rust-bin.stable.latest.default;
      in {
        # Run `nix develop` to get a reproducible dev shell
        devShells.default = pkgs.mkShell {
          buildInputs = [ rust ];
        };
        # Build nix packages with `nix build`
        packages = rec {
          default = hongdown;
          hongdown = let
            lib = pkgs.lib;
            rustPlatform = pkgs.rustPlatform;
            fetchFromGithub = pkgs.fetchFromGitHub;
          in
            import ./nix/hongdown.nix {inherit lib rustPlatform fetchFromGithub;};
        };
        # Run the program with `nix run`
        apps.default = self.packages.default;
      }
    );
}
