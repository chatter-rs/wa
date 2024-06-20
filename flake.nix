{
  description = "Flake for chatter-rs/wa";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    with pkgs;
    {

      devShells.default = mkShell {
        name = "chatter-rs-wa-dev";

        nativeBuildInputs = [
          fenix.packages."${system}".stable.toolchain
          protobuf
        ];
      };

    }
  );
}
