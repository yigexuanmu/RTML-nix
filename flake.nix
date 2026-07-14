{
  description = "RTML - Rusted TUI Minecraft Launcher (Nix flake)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rtml-src = {
      url = "github:MEKCCK/RTML";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      rtml-src,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.openjdk17
          ];

          shellHook = ''
            echo "RTML development shell"
            echo "  Source: ${rtml-src}"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rtml";
          version = "0.1.0";
          src = rtml-src;
          cargoLock.lockFile = "${rtml-src}/Cargo.lock";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.openjdk17
          ];
          buildInputs = [ ];
          JAVA_HOME = "${pkgs.openjdk17}";
          doCheck = false;
          meta = with pkgs.lib; {
            description = "A TUI Minecraft launcher with BMCLAPI mirror support";
            homepage = "https://github.com/MEKCCK/RTML";
            license = licenses.gpl3Plus;
            mainProgram = "rtml";
          };
        };
      }
    );
}
