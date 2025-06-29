{ inputs, ... }:

{
  perSystem = { pkgs, system, ... }: {
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
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
