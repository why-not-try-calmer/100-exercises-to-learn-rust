{
  inputs = {
    nixpkgs.url = "https://github.com/NixOS/nixpkgs/archive/54aac082a4d9.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { system = "x86_64-linux"; config.allowUnfree = true; };
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = [
            pkgs.gcc
          ];
          # shellHook = ''
          # 	export LD_LIBRARY_PATH = "${
          #     pkgs.lib.makeLibraryPath (with pkgs; [stdenv.cc.cc openssl_1_1 ] )
          # 	}:$LD_LIBRARY_PATH"
          # '';
        };
      }
    );
}
