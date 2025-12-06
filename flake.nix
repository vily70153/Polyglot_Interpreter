{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # 1. Добавляем оверлей для Rust
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # 2. Подключаем оверлей к nixpkgs
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # 3. Создаем тулчейн прямо из вашего файла rust-toolchain.toml
        # Это автоматически скачает Nightly и нужные компоненты
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        libPath = with pkgs; lib.makeLibraryPath [
          # load external libraries that you need in your rust project here
        ];
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = [ pkgs.pkg-config ];
          
          buildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            # 4. ВАЖНО: Убираем 'rustup' и ставим наш 'rustToolchain'
            rustToolchain 
            
            diesel-cli
            openssl
            zstd
            mysql80
            pkg-config
          ];

          # Больше не нужно вручную парсить TOML и менять PATH в shellHook,
          # rustToolchain сам всё сделает.

          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          
          OPENSSL_NO_VENDOR = 1; 
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

          # shellHook теперь нужен только для пользовательских настроек, 
          # манипуляции с RUSTUP_HOME можно убрать
          shellHook = ''
             # Опционально: полезно видеть, какая версия загрузилась
             echo "Rust version:"
             rustc --version
          '';

          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here
          ]);
          
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
          
          # Add glibc, clang, glib, and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
          (builtins.map (a: ''-I"${a}/include"'') [
            pkgs.glibc.dev
          ])
          ++ [
            ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
        };
      }
    );
}
