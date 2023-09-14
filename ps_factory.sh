#!/bin/zsh

echo "Enter the ps number:"
read ps_number

# Create the directory
ps_dir="./$ps_number"
if [ -d "$ps_dir" ]; then
    echo "Destination already exists"
    exit 1
fi
mkdir -p "$ps_dir"

# Create the Rust project directory
project_dir="$ps_dir/baejoon_$ps_number"
if [ ! -d "$project_dir" ]; then
    cargo new "$project_dir"
fi

# Create .gitignore file
gitignore_file="$project_dir/.gitignore"
echo "/target" > "$gitignore_file"

echo "Process.."
echo "Completed!"
