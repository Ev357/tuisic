{ lib, inputs, pkgs, system, ... }:

let
  toolchain = inputs.fenix.packages.${system}.minimal.toolchain;
in
(pkgs.makeRustPlatform {
  cargo = toolchain;
  rustc = toolchain;
}).buildRustPackage {
  pname = "tuisic";
  version = "0.1.0";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "Tuisic music player";
    homepage = "https://github.com/Ev357/tuisic";
    license = lib.licenses.mit;
    maintainers = [ lib.maintainers.ev357 ];
  };
}
