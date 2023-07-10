#!/bin/bash

# Prompt user for input
read -p "Enter password: " password
read -p "Enter universal path: " universal_path
read -p "Enter db settings URL: " db_settings_url

# Create TOML file
cat > config.toml <<EOF
[server_settings]
password = "$password"
universal_path = "$universal_path"
[db_settings]
db_settings_url = "$db_settings_url"
EOF

echo "TOML file created successfully!"

