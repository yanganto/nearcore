let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "f233fdc4ff6ba2ffeb1e3e3cd6d63bb1297d6996";
    });
  pinned = builtins.fetchGit {
    url = "https://github.com/nixos/nixpkgs/";
    ref = "refs/heads/release-21.11";
    rev = "f0b2ba0d57905e212218cb30624945a93d98ff5f";
  };
  nixpkgs = import pinned { overlays = [ mozillaOverlay ]; };
  toolchain = with nixpkgs; (rustChannelOf { version = "1.62.1"; channel = "stable"; });
  rust-wasm = toolchain.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
  };
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    clang
    pkg-config
    rocksdb
    rust-wasm
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library/";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
}
