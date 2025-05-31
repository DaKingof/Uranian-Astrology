{ pkgs ? import <nixpkgs> {} }:

let
  # Use Qt6 instead of Qt5
  qt6 = pkgs.qt6;
  
  # Create Qt6 environment with all necessary components
  qtEnv = with qt6; [
    qtbase
    qtdeclarative
    qt5compat
    qttools
    qmake
    qtshadertools
    qtsvg
    qtwayland
    qtmultimedia
  ];
  
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    
    # Build tools
    cmake
    ninja
    pkg-config
    
    # Qt6 tools
    qt6.qttools
    qt6.qmake
    qt6.qtbase
    
    # C++ compiler and tools
    clang
    llvmPackages.libclang
    gcc
    
    # Additional development tools
    git
    just
  ] ++ qtEnv;

  buildInputs = with pkgs; [
    # System libraries
    openssl
    zlib
    libxml2
  ] ++ qtEnv;

  # Environment variables for Qt6
  QT_QPA_PLATFORM_PLUGIN_PATH = "${qt6.qtbase}/${qt6.qtbase.qtPluginPrefix}/platforms";
  QML_IMPORT_PATH = "${qt6.qtdeclarative}/${qt6.qtbase.qtQmlPrefix}";
  QT_PLUGIN_PATH = "${qt6.qtbase}/${qt6.qtbase.qtPluginPrefix}";
  
  # For cxx-qt
  QTDIR = "${qt6.qtbase.dev}";
  QT_VERSION = "6";
  
  # Rust and C++ environment
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath ([
    stdenv.cc.cc.lib
    openssl
    zlib
  ] ++ qtEnv);
  
  shellHook = ''
    echo "🚀 Uranian Astrology Development Environment (Qt6 + Rust)"
    echo "📦 Qt version: $(qmake -query QT_VERSION)"
    echo "🦀 Rust version: $(rustc --version)"
    echo ""
    
    # Set up Rust environment
    export PATH="$HOME/.cargo/bin:$PATH"
  '';
}
