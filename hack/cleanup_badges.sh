#!/bin/bash

# Set paths to files
BADGES_CSV="./content/badges.csv"
BADGES_DIR="./assets/img/badges/"
MAX_PARALLEL=10  # Maximum number of parallel downloads

# Initialize arrays to store valid image paths and their sources
declare -A image_sources
valid_images=()

# Read the CSV file
echo "Reading data from $BADGES_CSV..."
while IFS=, read -r alt path link source; do
    # Skip the header line
    if [[ "$path" != "path" ]]; then
        # Remove any leading/trailing spaces or newlines
        path=$(echo "$path" | xargs)
        source=$(echo "$source" | xargs)

        if [[ -n "$path" ]]; then
            valid_images+=("$path")
            # Store source URL if it exists
            if [[ -n "$source" ]]; then
                image_sources["$path"]="$source"
            fi
        fi
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

# Create badges directory if it doesn't exist
mkdir -p "$BADGES_DIR"

# First, clean up unused files
echo "Checking for unused files in $BADGES_DIR..."
for file in "$BADGES_DIR"*; do
    if [[ -f "$file" ]]; then  # Check if it's a file
        filename=$(basename "$file")
        echo "Checking: $filename"

        if is_in_list "$filename"; then
            echo "File $filename is valid, keeping it for now!"
        else
            echo "File $filename is NOT in the list, removing it!"
            rm "$file"
        fi
    fi
done

# Download files in parallel with a maximum limit
echo "Downloading/updating files from source URLs..."
active_downloads=0

for image in "${valid_images[@]}"; do
    if [[ -n "${image_sources[$image]}" ]]; then
        target_path="${BADGES_DIR}${image}"

        # Start download in background
        (
            echo "Downloading $image from ${image_sources[$image]}"
            wget -q "${image_sources[$image]}" -O "$target_path"
            if [[ $? -eq 0 ]]; then
                echo "Successfully downloaded $image"
            else
                echo "Failed to download $image"
            fi
        ) &

        ((active_downloads++))

        # If we've reached the maximum parallel downloads, wait for one to finish
        if ((active_downloads >= MAX_PARALLEL)); then
            wait -n
            ((active_downloads--))
        fi
    else
        echo "No source URL for $image"
    fi
done

# Wait for any remaining downloads to complete
wait

echo "Cleanup and download complete! 🐾"
