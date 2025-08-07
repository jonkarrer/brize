{
  description = "Dev shell with Rust tools and Stripe CLI on multiple platforms";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git
            curl
            zsh
            zsh-syntax-highlighting
            zsh-autosuggestions
            rustc
            cargo
            rustfmt
            rust-analyzer
            helix
            docker
            colima
            stripe-cli
          ];

          shellHook = ''
            echo "Project Shell 🛸"
            export ZSH_SYNTAX_HIGHLIGHTING=${pkgs.zsh-syntax-highlighting}/share/zsh-syntax-highlighting
            export ZSH_AUTOSUGGESTIONS=${pkgs.zsh-autosuggestions}/share/zsh-autosuggestions
            # Only exec zsh if this shell is not zsh already:
            if [ ! "$SHELL" = "$(command -v zsh)" ]; then
              export ZDOTDIR="$HOME/devjon/configs/shells/project"
              exec zsh
            fi
          '';
        };
      }
    );
}
