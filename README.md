# Rust UI

This project is my attempt to build hardware-accelerated cross-platform user-interface entirely is rust.

**NOTE:** This project on early stage of development.

## Libraries

+ __sgl__ Safe thin and easy to use OpenGL (ES) layer
+ __apl__ Polymorhic and easy to use application layer

## Applications

+ __demo__ Simple GLES demo which only draw triangle on the screen
  and purposed to test context initialization on different platforms.

## NixOS tips

```sh
# Setup desktop environment (Unix)
$ nix-shell nix/unix-env.nix

# Setup android development (currently doesn't work because ndk is outdated)
$ nix-shell nix/android-env.nix

# Setup android development using sdk/ndk in $HOME/.androidenv
$ nix-shell nix/android-env-custom.nix

# Setup android emulator
$ [ANDROID=<platform>|16]
  [ANDROID_ABI=<arch>|x86]
  nix-build nix/android-emu.nix
$ result/bin/run-test-emulator
```
