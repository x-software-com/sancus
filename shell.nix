{
  pkgs ? (
    import <nixpkgs> {
      config.allowUnfree = true;
    }
  ),
  userShell ? "fish",
}:

let
  pkgConfigPath = "$PKG_CONFIG_PATH:/usr/lib/pkgconfig:/usr/share/pkgconfig";
  pkgConfigWrapper = pkgs.writeShellScriptBin "pkg-config" ''
    PKG_CONFIG_PATH=${pkgConfigPath} ${pkgs.pkg-config}/bin/pkg-config $@
  '';
in
(pkgs.buildFHSEnv {
  name = "mxl-plyr";
  targetPkgs =
    pkgs:
    (with pkgs; [
      pkgConfigWrapper
      vscode # Explicitly install vscode to run the non FSH version to inherit all environment variables
      pkgs.${userShell}
      rustup
      rustPlatform.bindgenHook
      stdenv
      gcc
      libgcc.lib
      just
      gdb
      pkg-config
      gitFull

      wget
      curl
      htop
      eza
      ripgrep
      bat
      dust
      fd
      ouch
      zip
      delta
    ]);

  runScript =
    with pkgs;
    pkgs.writeScript "init.sh" ''
      set -e

      rustup install stable
      rustup install nightly
      rustup default stable
      rustup update

      TEXT="Sancus Development Environment"
      LEN=$(($(set -e;echo $TEXT | wc -c) - 1))
      echo $(set -e;printf '%*s' $LEN "" | tr ' ' '=')
      echo $TEXT
      echo $(set -e;printf '%*s' $LEN "" | tr ' ' '-')
      echo "Rust version: $(set -e;rustc --version)"
      echo "Cargo version: $(set -e;cargo --version)"
      echo "GCC version: $(set -e;gcc --version | grep gcc)"
      echo "Nixpkgs version: ${pkgs.lib.version}"
      echo "Docker version: $(docker --version 2>/dev/null || echo 'Docker not available')"
      echo ""

      export PKG_CONFIG_PATH="${pkgConfigPath}"
      export PKG_CONFIG_EXECUTABLE="$(set -e; which pkg-config)"

      export SHELL="/usr/bin/${userShell}"
      exec ${userShell}
    '';

  profile = ''
    # Set the Cargo home directory to avoid conflicts with other projects and different compiler and library versions.
    export CARGO_HOME="${builtins.toString ./.}/.cargo-cache"

    # Set the rustup home directory to avoid conflicts with other projects and the system.
    export RUSTUP_HOME="${builtins.toString ./.}/.rustup";
  '';
}).env
