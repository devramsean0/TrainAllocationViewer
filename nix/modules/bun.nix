{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }:
    {
      devShells.bun = pkgs.mkShell {
        name = "train-allocation-viewer-bun-shell";
        inputsFrom = [
          self'.devShells.default
        ];
        packages = with pkgs; [
          bun
          tailwindcss_4
        ];

        shellHook = ''
        '';
      };
    };
}
