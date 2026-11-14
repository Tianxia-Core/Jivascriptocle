# ffp: FFP-JIVA-FLAKE declarative nix
{
  description = "Jivascriptocle — Deterministic Neurosymbolic Polyglot AST Transpiler, TOON Schema Compiler & ARSI Verifier (Apache-2.0)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "jivascriptocle";
            version = "1.2.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            meta = with pkgs.lib; {
              description = "Deterministic Neurosymbolic Polyglot AST Transpiler & ARSI Proof Engine";
              homepage = "https://github.com/Tianxia-Core/Jivascriptocle";
              license = licenses.asl20;
              mainProgram = "jivascriptocle";
            };
          };
        });

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              clippy
              rustfmt
              zig
              ocaml
              clang
              python3
            ];
          };
        });
    };
}
