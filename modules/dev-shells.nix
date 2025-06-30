{ inputs, ... }:

{
  perSystem = { pkgs, system, ... }:
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs.extend inputs.fenix.overlays.default; [
          (inputs.fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          rust-analyzer-nightly
        ];
      };
    };
}
