{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      ipfs
      nodePackages.ganache
      solc
    ];
  }