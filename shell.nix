{pkgs ? import <nixpkgs> {}}: pkgs.mkShell {
  buildInputs = [ pkgs.rustup pkgs.sqlx-cli pkgs.pkg-config pkgs.nodejs pkgs.wl-clipboard pkgs.asciinema];
  nativeBuildInputs = [ pkgs.curl ];
  DATABASE_URL = "sqlite:./target/db.sqlite";
}
