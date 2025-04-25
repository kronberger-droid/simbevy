{ pkgs ? import <nixpkgs> {} }:

let
  inherit (pkgs) lib;

  runtimeLibs = with pkgs; [
    wayland
    mesa       # <-- missing if you use any OpenGL internally
    libxkbcommon
    xorg.libX11 # <-- needed for x11 fallback paths inside winit
    systemd
    alsa-lib
    vulkan-loader
  ];

  withDev = pkg: if lib.hasAttr "dev" pkg then [ pkg pkg.dev ] else [ pkg ];

in
{
  nativeBuildInputs = with pkgs; [
    pkg-config
    rustup
    clang
    cmake
    nushell
  ];

  buildInputs = lib.flatten (map withDev runtimeLibs);

  LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";

  LD_LIBRARY_PATH = lib.makeLibraryPath runtimeLibs;

  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages_latest.libclang.lib}/lib"
    export LD_LIBRARY_PATH="${lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
    nu
  '';
}
