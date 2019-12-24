with import <nixpkgs> {};
#{ pkgs ? import <nixpkgs> {} }:
#with pkgs;
androidenv.emulateApp {
  name = "phone_4_1";
  platformVersion = "16";
  #useGoogleAPIs = false;
  enableGPU = false;
  abiVersion = "x86";
  avdHomeDir = "/home/kayo";
}
