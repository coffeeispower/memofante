{pkgs ? import <nixpkgs> {}}: pkgs.mkShell {
  buildInputs = [ pkgs.rustup pkgs.sqlx-cli pkgs.pkg-config ];
  nativeBuildInputs = [ pkgs.curl ];
  DATABASE_URL = "sqlite:./target/db.sqlite";
}
