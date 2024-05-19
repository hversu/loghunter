#!/bin/bash

# Check if keyword argument is provided
if [ $# -eq 0 ]; then
  echo "Usage: $0 <keyword>"
  exit 1
fi

# Assign the keyword argument to a variable
keyword=$1

# Temporary file to store concatenated results
temp_file=$(mktemp)

# Concatenate all .csv files into the temporary file
cat ~/collector/collected/*.csv > "$temp_file"

# Filter the concatenated results using the keyword and display them
grep "$keyword" "$temp_file"

# Clean up the temporary file
rm "$temp_file"
