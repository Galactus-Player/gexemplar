with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "exemplar-rs";

  buildInputs = [
    pkgs.swagger-codegen
    pkgs.openapi-generator-cli
    pkgs.cargo
    pkgs.rustc
    pkgs.openssl
    pkgconfig
  ];
}
