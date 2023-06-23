{
  description = "Leet your code in command-line.";

  inputs.nixpkgs.url      = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.utils.url        = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          dbus
          sqlite
        ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];


        package = with pkgs; rustPlatform.buildRustPackage rec {
          pname = "leetcode-cli";
          version = "0.3.11";
          src = fetchCrate {
            inherit pname version;
            sha256 = "sha256-DHtIhiRPRGuO6Rf1d9f8r0bMOHqAaJleUvYNyPiX6mc=";
          };
          cargoSha256 = "sha256-Suk/nQ+JcoD9HO9x1lYp+p4qx0DZ9dt0p5jPz0ZQB+k=";

          inherit buildInputs nativeBuildInputs;

          # a nightly compiler is required unless we use this cheat code.
          RUSTC_BOOTSTRAP = 0;

          # CFG_RELEASE = "${rustPlatform.rust.rustc.version}-stable";
          CFG_RELEASE_CHANNEL = "stable";

          meta = with pkgs.lib; {
            description = "Leet your code in command-line.";
            homepage = "https://github.com/clearloop/leetcode-cli";
            licenses = licenses.mit;
            maintainers = with maintainers; [ congee ];
            mainProgram = "leetcode";
          };
        };
      in
      {
        defaultPackage = package;
        overlay = final: prev: { leetcode-cli = package; };

        devShell = with pkgs; mkShell {
          name = "shell";
          inherit nativeBuildInputs;

          buildInputs = buildInputs ++ [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
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
