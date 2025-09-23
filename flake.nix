{
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux";
     pkgs = nixpkgs.legacyPackages.${system};
     common = {
        buildInputs = [ pkgs.openssl ];
        nativeBuildInputs = [ pkgs.pkg-config ];
     };
    in {
      devShells.${system}.default = pkgs.mkShell ({
        packages = with pkgs; [
          rustc
          cargo
        ];
      } // common);

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage ({
        name = "icomidal";
        src = ./.;
        cargoHash = "sha256-vaMAQmDT2ZCZTbI9Kp/5MOTbe1a1DFqTKlcQqvR4vxA=";
      } // common);
    };
}
