{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs.buildPackages; [ rust-bin.beta.latest.default ];
          packages = with pkgs; [
            cargo-watch
            sqlx-cli
            sqlite
            sqlfluff
            postgresql

            # Needed for rust reqwest crate
            # https://nixos.wiki/wiki/Rust
            openssl
            pkg-config

            # For the frontend
            nodejs
          ];

          env = {
            # DATABASE_URL = "sqlite:bruttan.db";
            # Format "postgres://<username>:<password>@<host>:<port>/<database_name>"
            DATABASE_URL = "postgres://postgres:postgres@localhost:5432/bruttan";
          };
        };
      }
    );
}
