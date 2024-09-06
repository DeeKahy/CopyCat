{ lib, rustPlatform, fetchFromGitHub }:

rustPlatform.buildRustPackage rec {
  pname = "ccat";
  version = "001"; 

  src = fetchFromGitHub {
    owner = "DeeKahy";
    repo = "CopyCat";
    rev = "001"; 
    sha256 = "sha256-zllxQifRMNEMa3RO5WKrwGAUf1xQg6YrQBzIHzy43F0="; # This hash may need to be updated 
    };

  cargoHash = "sha256-LYVhvq5l+PCZXW+elWi3zZFxLekgPn+plo4dybbLK9g="; # This hash may need to be updated 
  
  meta = with lib; {
    description = "A utility to copy project tree contents to clipboard";
    homepage = "https://github.com/DeeKahy/CopyCat";
    license = licenses.mit;
    maintainers = [ maintainers.deekahy ];
  };
}
