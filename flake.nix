
{
  description = "A robust, native Rust application to send notifications to all active graphical users.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Import overlays
        overlays = [ (import rust-overlay) ];
        # Get pkgs with overlays
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain setup
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Common dependencies for both the package and the dev shell
        commonBuildInputs = with pkgs; [
          # System library needed by the zbus crate to communicate with D-Bus
          dbus
          # System library needed for logind integration
          systemd
          # pkg-config is often required by build scripts to find system libraries
          pkg-config
        ];

      in
      {
        # Development Shell (`nix develop`)
        devShells.default = pkgs.mkShell {
          name = "notify-all-users-dev";

          # Build inputs are dependencies for the development environment
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
          ] ++ commonBuildInputs;

          # Environment variables for the shell
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          # Set the log level for development
          RUST_LOG = "info,notify_all_users=debug";
        };

        # Nix Package (`nix build`)
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "notify-all-users";
          version = "1.0.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          # Native build inputs are dependencies required to build the package
          nativeBuildInputs = with pkgs; [ pkg-config ];
          # Build inputs are link-time dependencies
          buildInputs = commonBuildInputs;

          # Check phase can be enabled for more rigorous testing
          doCheck = false;
        };

        # Default application to run (`nix run`)
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      }
    );
}

