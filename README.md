# nixgc-clear

CLI tool i made to more easily delete nix gc roots.

it only shows links that you have perms to delete, groups them by project (found with .git or .direnv) and prompts to delete.

you can either delete all links in project, or individually.

in the end execs into nix-collect-garbage if you want it to.

# Installation

This flake contains package called `nixgc-clear`.

# License

MIT. see `LICENSE`
