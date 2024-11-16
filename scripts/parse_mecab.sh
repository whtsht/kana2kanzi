#!/usr/bin/env bash

input_file="$1"

if [ -z "$input_file" ]; then
  echo "Usage: $0 <input_file>"
  exit 1
fi

if [ ! -f "$input_file" ]; then
  echo "File not found: $input_file"
  exit 1
fi

while IFS= read -r line; do
  mecab_output=$(echo "$line" | mecab -b 65536)
  echo "$mecab_output" | awk 'BEGIN {ORS=" "} {if ($1 != "EOS") print $1}'
  echo
done < "$input_file"
