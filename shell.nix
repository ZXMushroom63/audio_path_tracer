{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cmake
    pkgs.git
    pkgs.gcc
    pkgs.python3
    pkgs.pybind11
    pkgs.opensource.usd
    pkgs.python314Packages.setuptools
#    pkgs.python314Packages.distutils
    pkgs.rustc
    pkgs.cargo  
  ];
}