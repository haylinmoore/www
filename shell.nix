{ pkgs ? import <nixpkgs> {}}:

let
  # Create custom scripts as derivations
  startScript = pkgs.writeScriptBin "pstart" ''
    #!/bin/sh
    # Store PID file in the TMP directory
    PID_FILE="$TMP/edwood-pstart.pid"

    # Check if process is already running
    if [ -f "$PID_FILE" ] && kill -0 $(cat "$PID_FILE") 2>/dev/null; then
      echo "Process already running with PID $(cat "$PID_FILE")"
      exit 1
    fi

    # Start the process and store its PID
    ${pkgs.cargo}/bin/cargo run &
    PID=$!
    echo $PID > "$PID_FILE"
    echo "Process started with PID $PID"

    # Wait for the process to complete
    wait $PID
    rm -f "$PID_FILE"
  '';

  stopScript = pkgs.writeScriptBin "pstop" ''
    #!/bin/sh
    PID_FILE="$TMP/edwood-pstart.pid"

    # Check if PID file exists
    if [ ! -f "$PID_FILE" ]; then
      echo "No process found. Is it running?"
      exit 1
    fi

    PID=$(cat "$PID_FILE")

    # Check if process is still running
    if kill -0 $PID 2>/dev/null; then
      echo "Stopping process with PID $PID"
      kill $PID
      rm -f "$PID_FILE"
    else
      echo "Process is not running, cleaning up PID file"
      rm -f "$PID_FILE"
    fi
  '';

in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    ffmpeg

    # edwood
    edwood
    startScript
    stopScript
  ];

  RUST_BACKTRACE = 1;
  shellHook = ''
    function to_webp() {
      input_file="$1"
      output_file="''${input_file%.*}.webp"

      if [[ "''$input_file" == *.gif ]]; then
        ffmpeg -i "''$input_file" -pix_fmt bgra -c:v libwebp_anim -lossless 1 -loop 0 -compression_level 6 "''$output_file"
      else
        ffmpeg -i "''$input_file" -pix_fmt bgra -c:v libwebp -lossless 1 -compression_level 6 "''$output_file"
      fi
    }
  '';
}
