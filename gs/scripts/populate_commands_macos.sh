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

# Replace the NamedCommand type definition
sed -i '' "" "s|^export type NamedCommand.*|export type NamedCommand = $commands_union;|" ./src/lib/types.ts

# Replace the NamedCommandValues array
sed -i '' "" "s|^export const NamedCommandValues.*|export const NamedCommandValues:NamedCommand[] = [$commands_array];|" ./src/lib/types.ts

echo $commands_union
echo $commands_array
