import torch
from safetensors.torch import save_file

def load_and_save(model_path: str, output_path: str):
    model = torch.load(model_path)
    save_file(dict(model), output_path)

def save_files(
    encoder_path,
    encoder_output,
    vae_path,
    vae_output,
    unet_path,
    unet_output,
):
    load_and_save(encoder_path, encoder_output)
    load_and_save(vae_path, vae_output)
    load_and_save(unet_path, unet_output)

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--encoder_path", type=str, required=True)
    parser.add_argument("--encoder_output", type=str, required=True)
    parser.add_argument("--vae_path", type=str, required=True)
    parser.add_argument("--vae_output", type=str, required=True)
    parser.add_argument("--unet_path", type=str, required=True)
    parser.add_argument("--unet_output", type=str, required=True)

    args = parser.parse_args()

    save_files(
        args.encoder_path,
        args.encoder_output,
        args.vae_path,
        args.vae_output,
        args.unet_path,
        args.unet_output,
    )
