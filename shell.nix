{ pkgs ? import <nixpkgs> {} }:

let
  fhs = pkgs.buildFHSEnv {
    name = "fhs-env";
    targetPkgs = pkgs: with pkgs; [
      zlib
      glibc
      gcc.cc.lib
      zstd
    ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    gnumake
    python3
    gdb
    rustup
    qemu
    openocd
    usbutils
    picotool
    unzip
    fhs
  ];

  shellHook = ''
    export RUSTC_WRAPPER=""
    export PATH="$PWD/.bin:$PATH"

    alr() {
      $PWD/.bin/alr-fhs "$@"
    }
    export -f alr

    echo "RSC-V Kernel Dev Environment Loaded"
  '';
}