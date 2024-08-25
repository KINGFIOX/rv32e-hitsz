{
  description = "A flake to provide an environment for fpga";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        verilatorIncludePath = "${pkgs.verilator}/share/verilator/include";
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # rust
            clippy
            rustc
            cargo
            rustfmt
            rust-analyzer
            # c++
            clang
            llvm
            clang-tools
            verilator
            cmake
            ninja
            # utils
            qemu
            (with pkgsCross.riscv64; [ buildPackages.gcc buildPackages.gdb ])
            (with pkgsCross.riscv32; [ buildPackages.gcc buildPackages.gdb ])
            yosys
            verible
          ];

          RUST_BACKTRACE = 1;

          shellHook = ''
            export C_INCLUDE_PATH=${verilatorIncludePath}:$C_INCLUDE_PATH
            export CPLUS_INCLUDE_PATH=${verilatorIncludePath}:$CPLUS_INCLUDE_PATH
            export VERILATOR_HOME=${pkgs.verilator}
            export MAKEFLAGS="-j$(nproc)"
          '';
        };
      });
}

