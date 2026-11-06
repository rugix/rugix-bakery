#!/bin/sh
set -eu

apt-get install -y --no-install-recommends systemd-resolved iproute2 curl xdelta3

install -D -m 600 "$RECIPE_INPUTS_DIR/ssh-key" /root/.ssh/authorized_keys
chmod 700 /root/.ssh
passwd -l root
mkdir -p /etc/ssh/sshd_config.d
cat > /etc/ssh/sshd_config.d/10-key-only.conf <<'EOF'
PermitRootLogin prohibit-password
PasswordAuthentication no
KbdInteractiveAuthentication no
EOF

mkdir -p /etc/systemd/network
cat > /etc/systemd/network/20-ethernet.network <<'EOF'
[Match]
Name=en* eth*
[Network]
DHCP=yes
EOF
systemctl enable systemd-networkd systemd-resolved
ln -sf /run/systemd/resolve/stub-resolv.conf /etc/resolv.conf
: > /etc/machine-id
rm -f /etc/ssh/ssh_host_*

install -D -m 644 "$RECIPE_INPUTS_DIR/bundle-root" /etc/rugix/bundle-root.crt
cat > /etc/rugix/ctrl.toml <<'EOF'
[signatures]
roots = ["/etc/rugix/bundle-root.crt"]
EOF
cat > /etc/rugix/state.toml <<'EOF'
overlay = "in-memory"
EOF
ln -s /run/rugix/mounts/data /data
