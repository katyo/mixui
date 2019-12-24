{ pkgs ? import <nixpkgs> {} }:
with pkgs;
stdenv.mkDerivation rec {
  name = "mixui-app";

  ANDROID_HOME = "${builtins.getEnv "HOME"}/.androidenv";
  NDK_HOME = "${ANDROID_HOME}/ndk/20.1.5948944";

  LD_LIBRARY_PATH = "${zlib}/lib";

  buildInputs = [
    zlib
  ];
}
