{
  description = "Leet your code in command-line.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    rust-overlay,
    ...
    }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        darwinBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        buildInputs = with pkgs; [
          openssl
          dbus
          sqlite
        ] ++ darwinBuildInputs;

        package = naersk'.buildPackage rec {
          pname = "leetcode-cli";
          version = "git";

          src = ./.;
          doCheck = true; # run `cargo test` on build

          inherit buildInputs nativeBuildInputs;

          buildNoDefaultFeatures = true;

          buildFeatures = "git";

          meta = with pkgs.lib; {
            description = "Leet your code in command-line.";
            homepage = "https://github.com/clearloop/leetcode-cli";
            licenses = licenses.mit;
            maintainers = with maintainers; [ congee ];
            mainProgram = "leetcode";
          };

          # Env vars
          # a nightly compiler is required unless we use this cheat code.
          RUSTC_BOOTSTRAP = 0;

          # CFG_RELEASE = "${rustPlatform.rust.rustc.version}-stable";
          CFG_RELEASE_CHANNEL = "stable";
        };
      in
        {
        defaultPackage = package;
        overlay = final: prev: { leetcode-cli = package; };

        devShell = with pkgs; mkShell {
          name = "shell";
          inherit nativeBuildInputs;

          buildInputs = buildInputs ++ [
            toolchain
            cargo-edit
            cargo-bloat
            cargo-audit
            cargo-about
            cargo-outdated
          ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_BACKTRACE = "full";
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
