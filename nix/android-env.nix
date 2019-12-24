{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  androidComposition = androidenv.composeAndroidPackages {
    toolsVersion = "26.1.1";
    platformToolsVersion = "28.0.1";
    buildToolsVersions = [ "28.0.3" ];
    includeEmulator = false;
    emulatorVersion = "27.2.0";
    platformVersions = [ "16" "26" ];
    includeSources = false;
    includeDocs = false;
    includeSystemImages = false;
    systemImageTypes = [ "default" ];
    abiVersions = [ "x86" "x86_64" "armeabi-v7a" "arm64-v8a" ];
    cmakeVersions = [ "3.10.2" ];
    includeNDK = true;
    ndkVersion = "18.1.5063045";
    useGoogleAPIs = false;
    useGoogleTVAddOns = false;
    includeExtras = [
      #"extras;google;gcm"
    ];
  };
  android-ndk = androidComposition.ndk-bundle;
  android-sdk = androidComposition.androidsdk;
in stdenv.mkDerivation rec {
  name = "mixui-app";

  ANDROID_SDK_HOME = "${android-sdk}/libexec/android-sdk";
  NDK_HOME = "${android-ndk}/libexec/android-sdk/ndk-bundle";

  buildInputs = [
    android-sdk
    android-ndk
  ];
}
