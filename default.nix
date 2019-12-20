{ pkgs ? import <nixpkgs> {} }:
with pkgs;
stdenv.mkDerivation rec {
  name = "pianino";

  LD_LIBRARY_PATH = "${libglvnd}/lib:${stdenv.cc.cc.lib}/lib";
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

  buildInputs = with xorg; [
    stdenv
    pkgconfig
    stdenv.cc.cc.lib
    llvmPackages.libclang.lib
    libX11
    libXext
    libXinerama
    libXi
    libXrandr
    libXcursor
    libglvnd
    #libGLU_combined
    mesa.dev
    openssl
    fontconfig
    freetype
    bzip2
  ];
}
