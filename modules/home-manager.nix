{ inputs, ... }:

{
  flake.homeManagerModules.default = (
    { config, lib, pkgs, ... }:
    let
      cfg = config.programs.tuisic;
      tuisic = pkgs.callPackage ../pkgs/tuisic.nix { inherit inputs; };
    in
    {
      options.programs.tuisic = {
        enable = lib.mkEnableOption "tuisic music player";

        package = lib.mkOption {
          type = lib.types.package;
          default = tuisic;
          description = "The package to use";
        };

        configFile = lib.mkOption {
          type = lib.types.nullOr lib.types.path;
          default = null;
          description = "Path to a custom configuration file";
        };

        settings = lib.mkOption {
          type = lib.types.attrs;
          default = { };
          description = "Settings";
        };
      };

      config = lib.mkIf cfg.enable {
        home.packages = [ cfg.package ];

        xdg.configFile."tuisic/config.toml" = lib.mkIf (cfg.configFile != null || cfg.settings != { }) (
          if cfg.configFile != null then {
            source = cfg.configFile;
          } else {
            source = (pkgs.formats.toml { }).generate "config" cfg.settings;
          }
        );
      };
    }
  );
}
