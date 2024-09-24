{ pkgs, ... }:

let openapi-generator-cli-7_6_0 = pkgs.openapi-generator-cli.overrideAttrs (finalAttrs: previousAttrs: {
    version = "7.6.0";
    src = pkgs.fetchurl {
      url = "mirror://maven/org/openapitools/${finalAttrs.pname}/${finalAttrs.version}/${finalAttrs.jarfilename}";
      sha256 = "sha256-NQdL3TzfxGvpqQLhGlSj+qPK4eNOtmy9lZ0cgHC719c=";
    };
  });
in {
  packages = [
    openapi-generator-cli-7_6_0
    pkgs.openssl
    pkgs.pkg-config
    pkgs.vscode
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  pre-commit.hooks.rustfmt.enable = true;

  scripts.docker-build-mock.exec = ''
    docker build \
        --file $DEVENV_ROOT/mock/Dockerfile \
        --tag somfy-protect-api-mock:latest \
        $DEVENV_ROOT/mock
  '';
  scripts.openapi-generate.exec = ''
    openapi-generator-cli generate \
        -g rust \
        -o $DEVENV_ROOT/somfy-protect-openapi \
        -i $DEVENV_ROOT/somfy-protect-openapi/somfy-protect-openapi.json \
        -c $DEVENV_ROOT/somfy-protect-openapi/openapi-config.yml
  '';

  enterShell = ''
    ln -sf $RUST_SRC_PATH $DEVENV_STATE/rust-stdlib
  '';
}
