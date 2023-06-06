# gpt tools

This is a work in progress collection of utilities based around calling the OpenAI API.
It isn't intended for any serious use, but is a place where I can prototype using the API.
Additionally I will include other common commands that I will use when interacting with it.

## utilities

You can generate a string that contains the code from a given directory.
All files will be included with a separating line and the file name at the top.
In order to ignore files, you can include a `.gptignore` file which will filter out files.
This can be included in any directory (thanks to the `ignore` crate) and will be applied similarly to a .gitignore.

To use it, invoke the cli with a path to walk: `nix run github:justinrubek/gpt-toolkit#cli util generate-prompt .`
