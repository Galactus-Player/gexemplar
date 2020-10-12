with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "gexemplar";

  buildInputs = [
    pkgs.openapi-generator-cli
    pkgs.act
    pkgs.gcc
  ];
}
