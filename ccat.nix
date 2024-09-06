
{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "ccat";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A utility to copy project contents to clipboard";
    homepage = "https://github.com/YourUsername/CopyCat";
    license = licenses.mit;  # Adjust this to your project's license
    maintainers = [ maintainers.yourname ];
  };
}