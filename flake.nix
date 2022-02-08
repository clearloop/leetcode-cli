{
  description = "Leet your code in command-line.";

  inputs.nixpkgs.url      = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.utils.url        = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, rust-overlay, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlay ]; };

        platform = with pkgs; makeRustPlatform {
          rustc = rust-bin.nightly.latest.minimal;
          cargo = rust-bin.nightly.latest.minimal;
        };
        package = with pkgs; platform.buildRustPackage rec {
          pname = "leetcode-cli";
          version = "0.3.10";

          src = fetchCrate {
            inherit pname version;
            sha256 = "SkJLA49AXNTpiWZByII2saYLyN3bAAJTlCvhamlOEXA=";
          };

          cargoSha256 = "xhKF4qYOTdt8iCSPY5yT8tH3l54HdkOAIS2SBGzqsdo=";

          # a nightly compiler is required unless we use this cheat code.
          RUSTC_BOOTSTRAP = 0;

          # CFG_RELEASE = "${rustPlatform.rust.rustc.version}-nightly";
          CFG_RELEASE_CHANNEL = "ngihtly";

          nativeBuildInputs = [
            pkg-config
            rust-bin.stable.latest.default
          ];

          buildInputs = [
            openssl
            dbus
            sqlite
          ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

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
      }
    );
}
