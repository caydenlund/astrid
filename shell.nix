{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
    nativeBuildInputs = [
        clang
        lld
        pkg-config
    ];
    buildInputs = [
        udev alsa-lib vulkan-loader
        xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
        libxkbcommon wayland
    ];
    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
