{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    inherit (self'.packages) libtorch;
    # packages required for building the rust packages
    extraPackages = [
      pkgs.pkg-config
      pkgs.zlib
      libtorch
    ];
    withExtraPackages = base: base ++ extraPackages;

    craneLib = inputs.crane.lib.${system}.overrideToolchain self'.packages.rust-toolchain;

    common-build-args = rec {
      src = inputs.nix-filter.lib {
        root = ../.;
        include = [
          "crates"
          "Cargo.toml"
          "Cargo.lock"
        ];
      };

      pname = "generation-toolkit";

      nativeBuildInputs = withExtraPackages [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
      LIBTORCH = "${libtorch}";
    };

    deps-only = craneLib.buildDepsOnly ({} // common-build-args);

    packages = {
      default = packages.generation-toolkit;
      generation-toolkit = craneLib.buildPackage ({
          pname = "generation-toolkit";
          cargoArtifacts = deps-only;
          cargoExtraArgs = "--bin generation-toolkit";
          meta.mainProgram = "generation-toolkit";
        }
        // common-build-args);

      cargo-doc = craneLib.cargoDoc ({
          cargoArtifacts = deps-only;
        }
        // common-build-args);
    };

    checks = {
      clippy = craneLib.cargoClippy ({
          cargoArtifacts = deps-only;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        }
        // common-build-args);

      rust-fmt = craneLib.cargoFmt ({
          inherit (common-build-args) src;
        }
        // common-build-args);

      rust-tests = craneLib.cargoNextest ({
          cargoArtifacts = deps-only;
          partitions = 1;
          partitionType = "count";
        }
        // common-build-args);
    };
  in rec {
    inherit packages checks;

    apps = {
      generation-toolkit = {
        type = "app";
        program = pkgs.lib.getBin self'.packages.generation-toolkit;
      };
      default = apps.generation-toolkit;
    };

    legacyPackages = {
      cargoExtraPackages = extraPackages;
    };
  };
}
