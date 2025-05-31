{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "www";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-HH30SCItDEKXw+eMe8ZbpBnL2oGc+opPu9EKfEaYJgU=";

  nativeBuildInputs = with pkgs; [
    image_optim
  ];

  installPhase = ''
    mkdir -p $out/assets
    mkdir -p $out/content
    cp -r ./assets/* $out/assets/

    image_optim -r $out/assets/
  
    cp -r ./content/* $out/content/
  '';
}

