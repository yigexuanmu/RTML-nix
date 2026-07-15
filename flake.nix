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

        rtmlCargo = builtins.fromTOML (builtins.readFile "${rtml-src}/Cargo.toml");

        rtml-unwrapped = pkgs.rustPlatform.buildRustPackage {
          pname = "rtml-unwrapped";
          version = rtmlCargo.package.version;
          src = rtml-src;
          cargoLock.lockFile = "${rtml-src}/Cargo.lock";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.makeWrapper
            pkgs.jdk17
          ];
          buildInputs = [ ];
          doCheck = false;

          postInstall = ''
            # No Java wrapping - users must provide their own JDK in PATH
          '';

          meta = with pkgs.lib; {
            description = "A TUI Minecraft launcher with BMCLAPI mirror support";
            homepage = "https://github.com/MEKCCK/RTML";
            license = licenses.gpl3Plus;
            mainProgram = "rtml";
          };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.jdk17
          ];

          shellHook = ''
            echo "RTML development shell"
            echo "  Source: ${rtml-src}"
          '';
        };

        packages = {
          default = rtml-unwrapped;
          inherit rtml-unwrapped;
        };
      }
    );
}
