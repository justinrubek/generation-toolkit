{self, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    inherit (self'.packages) python;

    scripts = {
      encode_weights = pkgs.writeShellApplication {
        name = "encode_weights";
        runtimeInputs = [python];
        text = ''
          python ${self}/bin/encode_weights.py "$@"
        '';
      };
    };

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

      dictionary = pkgs.fetchurl {
        url = "https://github.com/openai/CLIP/raw/main/clip/bpe_simple_vocab_16e6.txt.gz";
        sha256 = "sha256-kkaRrCiOVECSNhFWUq1KolD0ggPeUKnkcipuzUjWgEo=";
      };
    };

    # output derivation resulting from calling "scripts/encode_weights with the above bin files as args
    encoded = pkgs.runCommand "stability-models" {} ''
      mkdir -p $out

      ${scripts.encode_weights}/bin/encode_weights \
        --encoder_path ${bins.encoder} \
        --encoder_output $out/text_encoder.safetensors \
        --vae_path ${bins.vae} \
        --vae_output $out/vae.safetensors \
        --unet_path ${bins.unet} \
        --unet_output $out/unet.safetensors

      cp ${bins.dictionary} $out/dictionary.txt.gz
      gunzip $out/dictionary.txt.gz
    '';
  in {
    packages =
      {
        stable_diffusion2-1 = encoded;
      }
      // scripts;
  };
}
