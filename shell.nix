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
      perl
      gitFull
      delta
      stdenv
      gcc
      libgcc.lib
      valgrind
      python3Full
      pipx
      just
      gdb
      pkg-config
    ]);

  runScript =
    with pkgs;
    pkgs.writeScript "init.sh" ''
      echo "=============================="
      echo "Sancus Development Environment"
      echo "------------------------------"
      echo "Rust version: $(rustc --version)"
      echo "Cargo version: $(cargo --version)"
      echo "GCC version: $(gcc --version | grep gcc)"
      echo "Python version: $(python3 --version)"
      echo "Nixpkgs version: ${pkgs.lib.version}"
      echo "Docker version: $(docker --version 2>/dev/null || echo 'Docker not available')"
      echo ""

      # Set the Cargo home directory to avoid conflicts with other projects and different compiler and library versions.
      export CARGO_HOME="${builtins.toString ./.}/.cargo-cache"

      export PKG_CONFIG_PATH="${pkgConfigPath}"
      export PKG_CONFIG_EXECUTABLE="$(which pkg-config)"

      export SHELL="/usr/bin/${userShell}"
      exec ${userShell}
    '';
}).env
