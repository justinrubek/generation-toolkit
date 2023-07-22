{
  self,
  inputs,
  ...
}: {
  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    python-packages = (
      ps:
        with ps; [
          torch
          safetensors
        ]
    );

    python = pkgs.python3.withPackages python-packages;

    scripts = {
      encode_weights = pkgs.writeShellApplication {
        name = "encode_weights";
        runtimeInputs = [python];
        text = ''
          python ${self}/bin/encode_weights.py "$@"
        '';
      };
    };

    packages =
      {
        inherit python;
      }
      // scripts;
  in {
    inherit packages;
  };
}
