{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
      jq
      protobuf
      grpcurl
    ];
  }

