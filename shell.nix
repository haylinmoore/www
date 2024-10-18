{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    ffmpeg
  ];

  RUST_BACKTRACE = 1;
  shellHook = ''
    function to_webp() {
      input_file="''$1"
      output_file="''${input_file%.*}.webp"

      if [[ "''$input_file" == *.gif ]]; then
        ffmpeg -i "''$input_file" -pix_fmt bgra -c:v libwebp_anim -lossless 1 -compression_level 6 "''$output_file"
      else
        ffmpeg -i "''$input_file" -pix_fmt bgra -c:v libwebp -lossless 1 -compression_level 6 "''$output_file"
      fi
    }
  '';
}

