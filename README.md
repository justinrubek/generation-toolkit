# generation-toolkit

This is a work in progress collection of utilities for generating text/images.
This includes calling the OpenAI API as well as some local generation when possible.
This started as a place where I could prototype using OpenAI APIs and is expanding as I see fit.

## utilities

### stable diffusion

A nix flake package has been exposed that will help you get the weights needed to run stable diffusion.
You can use nix to create the symlink that the cli will expect for stable diffusion commands: `nix build .#stable-diffusion2-1 -o models`.
After doing this you may generate an image: `nix run github:justinrubek/generation-toolkit#cli stable-diff generate --prompt "A crab holding a flaming torch"`

### openai

You can generate a string that contains the code from a given directory.
All files will be included with a separating line and the file name at the top.
In order to ignore files, you can include a `.gptignore` file which will filter out files.
This can be included in any directory (thanks to the `ignore` crate) and will be applied similarly to a .gitignore.

To use it, invoke the cli with a path to walk: `nix run github:justinrubek/generation-toolkit#cli util generate-prompt .`
