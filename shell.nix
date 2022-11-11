with import <nixpkgs> { };

mkShell rec {
  packages = [ rust-analyzer ];
}
