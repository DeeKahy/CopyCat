{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "ccat";
  version = "0.1.0"; # Update this as needed

  src = fetchFromGitHub {
    owner = "DeeKahy";
    repo = "CopyCat";
    rev = "main"; # Or use a specific commit hash or tag
    sha256 = "sha256-K5cs2/yGZNANjrSvMqEfH0aus/KQgM8V4AmXZCnqb6k="; # Replace with the correct hash
  };

  cargoHash = "sha256-STpKWRTO6ahBgTsHZc47M3WIAonK9et4nYJvo0zrKPk="; # Set to empty string initially

  meta = with lib; {
    description = "A utility to copy project tree contents to clipboard";
    homepage = "https://github.com/DeeKahy/CopyCat";
    license = licenses.mit; # Adjust if your license is different
    maintainers = [ maintainers.deekahy ];
  };
}
