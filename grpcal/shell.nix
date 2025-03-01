{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
      jq
      protobuf
      google-cloud-sdk
      grpcurl
    ];
  }

