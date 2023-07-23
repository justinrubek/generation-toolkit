{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    sources = pkgs.fetchzip {
      name = "torch-rocm";
      url = "https://download.pytorch.org/libtorch/rocm5.4.2/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Brocm5.4.2.zip";
      sha256 = "sha256-jhKkcUWzPO5KyoTCQndxDi8iT6qzZqyBsPfWwM4G4G0=";
    };

    nativeBuildInputs = [
      pkgs.patchelf
      pkgs.addOpenGLRunpath
    ];

    torch-bin = pkgs.stdenv.mkDerivation rec {
      version = "2.0.1+rocm5.4.2";
      pname = "libtorch";
      name = "libtorch-${version}";

      src = sources;

      inherit nativeBuildInputs;

      dontBuild = true;
      dontConfigure = true;
      dontStrip = true;

      installPhase = ''
        # Copy headers and CMake files
        mkdir -p $out
        cp -r include $out
        cp -r share $out

        install -Dm755 -t $out/lib lib/*${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}*

        # Discard Java support
        rm -f $out/lib/lib*jni* 2> /dev/null || true

             # Fix up library paths for split outputs
        substituteInPlace $out/share/cmake/Torch/TorchConfig.cmake \
          --replace \''${TORCH_INSTALL_PREFIX}/lib "$out/lib" \

        substituteInPlace \
          $out/share/cmake/Caffe2/Caffe2Targets-release.cmake \
          --replace \''${_IMPORT_PREFIX}/lib "$out/lib" \
      '';

      postFixup = let
        rpath = lib.makeLibraryPath [pkgs.stdenv.cc.cc.lib];
      in ''
        find $out/lib -type f \( -name '*.so' -or -name '*.so.*' \) | while read lib; do
          echo "setting rpath for $lib..."
          patchelf --set-rpath "${rpath}:$out/lib" "$lib"
          addOpenGLRunpath "$lib"
        done
      '';

      meta = with lib; {
        description = "Torch C++ libraries";
        homepage = "https://pytorch.org/";
        license = licenses.bsd3;
        platforms = platforms.linux;
        maintainers = with maintainers; [self];
      };
    };
  in {
    packages = {
      libtorch = torch-bin;
      libtorch-source = sources;
    };
  };
}
