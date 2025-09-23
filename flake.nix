{
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux";
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [
          rustc
          cargo
        ];
        buildInputs = with pkgs; [
          openssl
        ];
        nativeBuildInputs = [ pkgs.pkg-config ];
        # env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rust}";
      };
    };
}
