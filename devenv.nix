{ pkgs, ... }:

{
  packages = [
    pkgs.openapi-generator-cli
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
