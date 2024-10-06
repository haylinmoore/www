#!/bin/bash

# Set paths to files
BADGES_CSV="./content/badges.csv"
BADGES_DIR="./assets/img/badges/"

# Initialize an array to store valid image paths
valid_images=()

# Read the list of image paths from the CSV file (2nd column after 'path')
echo "Reading valid images from $BADGES_CSV..."
while IFS=, read -r alt path link; do
  # Skip the header line
  if [[ "$path" != "path" ]]; then
    # Remove any leading/trailing spaces or newlines
    path=$(echo "$path" | xargs)
    valid_images+=("$path")
  fi
done < "$BADGES_CSV"

# Debug: Print out the valid images list
echo "Valid images list:"
for image in "${valid_images[@]}"; do
  echo "$image"
done

# Function to check if a file exists in the valid images list
function is_in_list() {
  local file=$1
  for image in "${valid_images[@]}"; do
    if [[ "$image" == "$file" ]]; then
      return 0
    fi
  done
  return 1
}

# Loop through all images in the badges directory
echo "Checking files in $BADGES_DIR..."
for file in "$BADGES_DIR"*; do
  filename=$(basename "$file")

  # Debug: Show the file being checked
  echo "Checking: $filename"

  # If the file is not in the valid images list, remove it
  if is_in_list "$filename"; then
    echo "File $filename is valid, keeping it!"
  else
    echo "File $filename is NOT in the list, removing it!"
    rm "$file"
  fi
done

echo "Cleanup complete! 🐾"
