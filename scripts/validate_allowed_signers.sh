#!/usr/bin/env bash
set -eou pipefail

# validate_allowed_signers.sh
# Validates the strict governance formatting of the allowed_signers file.
#
# Usage:
#   ./scripts/validate_allowed_signers.sh [file_path]

FILE_PATH=${1:-".github/trust/allowed_signers"}

if [[ ! -f "$FILE_PATH" ]]; then
    echo "Error: allowed_signers file not found at $FILE_PATH"
    exit 1
fi

echo "Validating allowed_signers at $FILE_PATH..."

declare -A IDENTITIES

while IFS= read -r line || [[ -n "$line" ]]; do
    # Skip empty lines and comments
    if [[ -z "${line// }" || "$line" =~ ^# ]]; then
        continue
    fi

    # Check for revoked marker
    if [[ "$line" == "revoked "* ]]; then
        line="${line#revoked }" # Remove the revoked prefix for parsing
    fi

    # Parse components
    # format: <identity> <keytype> <key> [comment]
    read -r identity keytype keypart rest <<< "$line"

    if [[ -z "$identity" || -z "$keytype" || -z "$keypart" ]]; then
        echo "[ERROR] Malformed line: $line"
        exit 1
    fi

    if [[ "$keytype" != "ssh-ed25519" ]]; then
        echo "[ERROR] Invalid key type '$keytype' for identity '$identity'. Only ssh-ed25519 is permitted."
        exit 1
    fi
    
    if [[ -n "${IDENTITIES[$identity]:-}" ]]; then
        echo "[ERROR] Duplicate identity found: $identity"
        exit 1
    fi

    IDENTITIES[$identity]=1

done < "$FILE_PATH"

# Finally, ensure ssh-keygen can parse the file without errors
if ! ssh-keygen -l -f "$FILE_PATH" > /dev/null 2>&1; then
    echo "[ERROR] ssh-keygen failed to parse $FILE_PATH"
    exit 1
fi

echo "[SUCCESS] allowed_signers passed strict validation."
exit 0
