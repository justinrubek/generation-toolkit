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

    packages = {
      inherit python;
    };
  in {
    inherit packages;
  };
}
