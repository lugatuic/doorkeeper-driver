with import <nixpkgs> { };

mkShell rec {
  packages = [ gcc gdb pkg-config libxkbcommon rust-analyzer ];
}
