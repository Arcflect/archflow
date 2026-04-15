#!/usr/bin/env bash
set -eou pipefail

# test_validate_allowed_signers.sh
# Tests the strict governance formatting validation for allowed_signers.

echo "Running allowed_signers validation tests..."

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

VALIDATOR="./scripts/validate_allowed_signers.sh"

# Generate mock keys
ssh-keygen -t ed25519 -N "" -f "$TMP_DIR/valid_key" -q
VALID_PUB=$(cat "$TMP_DIR/valid_key.pub")

ssh-keygen -t rsa -b 2048 -N "" -f "$TMP_DIR/rsa_key" -q
RSA_PUB=$(cat "$TMP_DIR/rsa_key.pub")

# Test 1: Valid file
echo "TEST 1: Valid file"
cat <<EOF > "$TMP_DIR/test1_signers"
governance@arcflect.com $VALID_PUB
EOF

if $VALIDATOR "$TMP_DIR/test1_signers" > /dev/null; then
    echo "  [PASS] Valid file accepted."
else
    echo "  [FAIL] Valid file rejected."
    exit 1
fi

# Test 2: Invalid key type
echo "TEST 2: Invalid key type (RSA)"
cat <<EOF > "$TMP_DIR/test2_signers"
bad@example.com $RSA_PUB
EOF

if ! $VALIDATOR "$TMP_DIR/test2_signers" > /dev/null 2>&1; then
    echo "  [PASS] Invalid key type (RSA) rejected."
else
    echo "  [FAIL] Invalid key type (RSA) accepted."
    exit 1
fi

# Test 3: Duplicate identity
echo "TEST 3: Duplicate identity"
cat <<EOF > "$TMP_DIR/test3_signers"
governance@arcflect.com $VALID_PUB
governance@arcflect.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIdummy...
EOF

if ! $VALIDATOR "$TMP_DIR/test3_signers" > /dev/null 2>&1; then
    echo "  [PASS] Duplicate identity rejected."
else
    echo "  [FAIL] Duplicate identity accepted."
    exit 1
fi

# Test 4: Malformed line
echo "TEST 4: Malformed line"
cat <<EOF > "$TMP_DIR/test4_signers"
only_identity_no_key
EOF

if ! $VALIDATOR "$TMP_DIR/test4_signers" > /dev/null 2>&1; then
    echo "  [PASS] Malformed line rejected."
else
    echo "  [FAIL] Malformed line accepted."
    exit 1
fi

echo ""
echo "All validation script tests passed successfully."
exit 0
