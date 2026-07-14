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

        jdks = [
          pkgs.openjdk25
          pkgs.openjdk21
          pkgs.openjdk17
          pkgs.openjdk8
        ];

        rtmlCargo = builtins.fromTOML (builtins.readFile "${rtml-src}/Cargo.toml");

        rtml-unwrapped = pkgs.rustPlatform.buildRustPackage {
          pname = "rtml-unwrapped";
          version = rtmlCargo.package.version;
          src = rtml-src;
          cargoLock.lockFile = "${rtml-src}/Cargo.lock";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.makeWrapper
          ] ++ (with pkgs; [
            openjdk17
          ]);
          buildInputs = [ ];
          JAVA_HOME = "${pkgs.openjdk17}";
          doCheck = false;

          postInstall = ''
            wrapProgram $out/bin/rtml \
              --set JAVA8 ${pkgs.openjdk8}/bin/java \
              --set JAVA17 ${pkgs.openjdk17}/bin/java \
              --set JAVA21 ${pkgs.openjdk21}/bin/java \
              --set JAVA25 ${pkgs.openjdk25}/bin/java \
              --prefix RTML_JAVA_PATHS : ${pkgs.lib.makeSearchPath "bin/java" jdks} \
              --set LD_LIBRARY_PATH ${pkgs.lib.makeLibraryPath [
                pkgs.openjdk17
              ]}
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
            pkgs.openjdk17
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
