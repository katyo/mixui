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
    #lldbVersions = [ "3.1.4508709" ];
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
  name = "pianino";

  #ANDROID_SDK_HOME = "${android-sdk}/libexec/android-sdk";
  #NDK_HOME = "${android-ndk}/libexec/android-sdk/ndk-bundle";

  ANDROID_HOME = "/home/kayo/.androidenv";
  NDK_HOME = "${ANDROID_HOME}/ndk/20.1.5948944";

  LD_LIBRARY_PATH = "${zlib}/lib";

  buildInputs = [
    #stdenv
    #pkgconfig
    zlib
    #android-sdk
    #android-ndk
  ];
}
