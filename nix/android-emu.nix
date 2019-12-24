with import <nixpkgs> {};
let platform_from_env = builtins.getEnv "ANDROID";
    platform = if platform_from_env == ""
        then "16" else platform_from_env;
    abi_from_env = builtins.getEnv "ANDROID_ABI";
    abi = if abi_from_env == ""
        then "x86" else abi_from_env;
in androidenv.emulateApp rec {
  name = "android${platform}";
  platformVersion = platform;
  enableGPU = true;
  abiVersion = abi;
  avdHomeDir = builtins.getEnv "HOME";
}
