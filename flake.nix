{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        code-analyzer = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          nativeBuildInputs = [ pkgs.cargo-nextest pkgs.maven pkgs.pkg-config pkgs.openssl ];
          buildInputs = [ pkgs.maven ];
          buildFlags = [ "--release" "--no-default-features" ];
          cargoTestCommands = x: [ ''cargo nextest run'' ];
        };
        dockerImage = pkgs.dockerTools.buildImage {
          name = "code-analyzer";
          tag = "latest";
          copyToRoot = with pkgs.dockerTools; [
            usrBinEnv
            pkgs.bash
            pkgs.coreutils
            code-analyzer
          ];
          config = {
            entrypoint = [ "/bin/code-analyzer" ];
          };
        };
      in
      rec {
        packages = {
          docker = dockerImage;
          default = code-analyzer;
          app = code-analyzer;
        };
        devShells = with pkgs;{
          default = mkShell {
            buildInputs = [ openssl cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer cargo-nextest jq maven pkg-config ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        };
      });
}
