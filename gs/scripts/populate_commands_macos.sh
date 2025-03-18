#!/bin/sh

echo 'Populating commands...'

# Get the commands from the commands.toml file
commands=$(awk -F 'name = "' '/name = / {print $2}' ../config/commands.toml | sed 's/"$//')

# Check if we got any commands
if [ -z "$commands" ]; then
    echo "Error: No commands found in ../config/commands.toml"
    exit 1
fi

# Add quotes around each command name
commands=$(echo "$commands" | sed -E 's/(.*)/"\1"/')

# Format into TS union
commands_union=$(echo "$commands" | sed -E ':a;N;$!ba;s/\n/ | /g')

# Format into TS array
commands_array=$(echo "$commands" | sed -E ':a;N;$!ba;s/\n/, /g')

# Use a temporary file to ensure safe modifications
tmp_file=$(mktemp)

# Step 1: Delete the content after the AUTO GENERATED comment
awk '!/AUTO GENERATED USING npm run generate:commands/ {print} /AUTO GENERATED USING npm run generate:commands/ {print; next}' ./src/lib/types.ts > "$tmp_file" && mv "$tmp_file" ./src/lib/types.ts

# Step 2: Insert the new content after the comment
awk -v commands_union="$commands_union" -v commands_array="$commands_array" '
    /AUTO GENERATED USING npm run generate:commands/ {
        print;
        print "export type NamedCommand = " commands_union ";";  # Insert union type
        print "export const NamedCommandValues: NamedCommand[] = [" commands_array "];";  # Insert array
        next;
    }
    { print }
' ./src/lib/types.ts > "$tmp_file" && mv "$tmp_file" ./src/lib/types.ts

echo "Updated NamedCommand with: $commands_union"
echo "Updated NamedCommandValues with: $commands_array"
