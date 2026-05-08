{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    treefmt-nix,
    git-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      rust = fenix.packages.${system};

      # ホストとWASMで同じRustツールチェーンを使う。
      rustWithWasm = rust.combine [
        (rust.latest.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-std"
        ])
        rust.targets.wasm32-unknown-unknown.latest.rust-std
      ];

      treefmt = treefmt-nix.lib.evalModule pkgs {
        projectRootFile = "flake.nix";
        programs.alejandra.enable = true;
        programs.rustfmt = {
          enable = true;
          package = rust.latest.rustfmt;
          edition = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.edition;
        };
        settings.global.excludes = ["target/**" "dist/**" ".direnv/**"];
      };

      preCommit = git-hooks.lib.${system}.run {
        src = self;
        hooks.treefmt = {
          enable = true;
          packageOverrides.treefmt = treefmt.config.build.wrapper;
          files = "\\.(nix|rs)$";
        };
      };
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs;
          [
            rustWithWasm
            treefmt.config.build.wrapper
            trunk
            tailwindcss
            pkg-config
            wasm-bindgen-cli
            binaryen
          ]
          ++ preCommit.enabledPackages;

        buildInputs = [pkgs.openssl];

        shellHook = ''
          ${preCommit.shellHook}
          export RUST_SRC_PATH="${rustWithWasm}/lib/rustlib/src/rust/library"
          # cargoのサブコマンドが既存のrustup経由でも同じツールチェーンを使う。
          export RUSTUP_TOOLCHAIN="${rustWithWasm}"
        '';
      };

      formatter = treefmt.config.build.wrapper;
      checks.formatting = treefmt.config.build.check self;
      checks.pre-commit = preCommit;
    });
}
