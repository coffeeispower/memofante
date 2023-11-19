{
  description = "memofante development environment";
  inputs = { nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; };
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default =
        pkgs.mkShell {
            buildInputs = [ pkgs.rustup pkgs.pkg-config pkgs.asciinema];
            nativeBuildInputs = [ pkgs.curl ];
        };
    };
}
