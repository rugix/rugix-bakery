#!/bin/sh
set -eu
umask 077
cd "$(dirname "$0")"

if [ -e keys ] || [ -L keys ]; then
    echo "keys already exists; keep it to build updates for the same devices." >&2
    exit 1
fi

key_dir=$(mktemp -d .keys.XXXXXX)
trap 'rm -rf "$key_dir"' EXIT HUP INT TERM

openssl req -new -x509 -newkey rsa:2048 -nodes -sha256 -days 3650 \
    -subj "/CN=Rugix development bundle root/" \
    -addext "basicConstraints=critical,CA:TRUE" \
    -addext "keyUsage=critical,keyCertSign,cRLSign" \
    -keyout "$key_dir/bundle-root.key" -out "$key_dir/bundle-root.crt"
openssl req -new -newkey rsa:2048 -nodes -sha256 \
    -subj "/CN=Rugix development bundle signer/" \
    -keyout "$key_dir/bundle.key" -out "$key_dir/bundle.csr"
cat > "$key_dir/bundle.extensions" <<'EXT'
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature
EXT
openssl x509 -req -in "$key_dir/bundle.csr" \
    -CA "$key_dir/bundle-root.crt" -CAkey "$key_dir/bundle-root.key" \
    -CAcreateserial -days 3650 -sha256 -extfile "$key_dir/bundle.extensions" \
    -out "$key_dir/bundle.crt"
rm "$key_dir/bundle.csr" "$key_dir/bundle.extensions" "$key_dir/bundle-root.srl"

openssl genrsa -out "$key_dir/boot.key" 2048
openssl rsa -in "$key_dir/boot.key" -pubout -out "$key_dir/boot.pub"

ssh-keygen -q -t ed25519 -N "" -f "$key_dir/ssh"
chmod 600 "$key_dir/"*.key "$key_dir/ssh"
chmod 644 "$key_dir/"*.crt "$key_dir/"*.pub
mv "$key_dir" keys
