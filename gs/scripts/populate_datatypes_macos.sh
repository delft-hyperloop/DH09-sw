#!/bin/sh

echo 'Populating datatypes...'

# Extract datatype names from datatypes.toml
datatypes=$(awk -F 'name = "' '/name = / {print $2}' ../config/datatypes.toml | sed 's/"$//')

# Check if we got any datatypes
if [ -z "$datatypes" ]; then
    echo "Error: No datatypes found in ../config/datatypes.toml"
    exit 1
fi

# Add quotes around each datatype name
datatypes=$(echo "$datatypes" | sed -E 's/(.*)/"\1"/')

# Format into TS union type (trim trailing `|`)
datatypes=$(echo "$datatypes" | tr '\n' '|' | sed 's/|/ | /g' | sed 's/ | $//')

# Delete the line after AUTO GENERATED comment
sed -i '' '/AUTO GENERATED USING npm run generate:datatypes/{n;d;}' ./src/lib/types.ts

# Use a temporary file to ensure safe modifications
tmp_file=$(mktemp)
awk -v datatypes="$datatypes" '
    /AUTO GENERATED USING npm run generate:datatypes/ {
        print;
        print "export type NamedDatatype = " datatypes ";";  # Insert line after match
        next;
    }
    { print }
' ./src/lib/types.ts > "$tmp_file" && mv "$tmp_file" ./src/lib/types.ts

echo "Updated NamedDatatype with: $datatypes"
