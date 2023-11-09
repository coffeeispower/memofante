{pkgs ? import <nixpkgs> {}}: pkgs.mkShell {
  buildInputs = [ pkgs.rustup pkgs.pkg-config pkgs.asciinema];
  nativeBuildInputs = [ pkgs.curl ];
  DATABASE_URL = "sqlite:./target/db.sqlite";
}
