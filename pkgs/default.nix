{ inputs, ... }:

{
  perSystem = { pkgs, config, ... }: {
    packages.default = config.packages.tuisic;

    packages.tuisic = pkgs.callPackage ./tuisic.nix { inherit inputs; };
  };
}
