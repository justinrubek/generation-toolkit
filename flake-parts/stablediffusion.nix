{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    inherit (self'.packages) encode_weights;
    # bin files needed for the model
    bins = {
      encoder = pkgs.fetchurl {
        url = "https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/text_encoder/pytorch_model.bin";
        sha256 = "sha256-XuZ/E95xnmxyM0tKBmhS1D3eGwULEbeJg/EvX41VU9g=";
      };

      vae = pkgs.fetchurl {
        url = "https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/vae/diffusion_pytorch_model.bin";
        sha256 = "sha256-NU6UzrjOvnIMQFELlFmbOIqO2kB+dpSk2x7Y3E9nhTg=";
      };

      unet = pkgs.fetchurl {
        url = "https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/unet/diffusion_pytorch_model.bin";
        sha256 = "sha256-AHwjJvN8RyRNttt8zmwwX+i5S3P7LSYOzKnvfZYGRMQ=";
      };
    };

    # output derivation resulting from calling "scripts/encode_weights with the above bin files as args
    encoded = pkgs.runCommand "stability-models" {} ''
      mkdir -p $out

      ${encode_weights}/bin/encode_weights \
        --encoder_path ${bins.encoder} \
        --encoder_output $out/text_encoder.safetensors \
        --vae_path ${bins.vae} \
        --vae_output $out/vae.safetensors \
        --unet_path ${bins.unet} \
        --unet_output $out/unet.safetensors
    '';
  in {
    packages = {
      stable_diffusion2-1 = encoded;
    };
  };
}
