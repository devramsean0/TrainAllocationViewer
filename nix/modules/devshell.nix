{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "train-allocation-viewer-shell";
      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];
      packages = with pkgs; [
        just
        nixd # Nix language server
        bacon
        openssl
        pkg-config
        wrapGAppsHook4

        sqlx-cli
        sqlite
        cargo-tauri

        librsvg
        webkitgtk_4_1
      ];

      shellHook = ''
        export PATH="$HOME/.cargo/bin:$PATH"
      '';
    };
  };
}
