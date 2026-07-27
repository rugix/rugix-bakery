#!/bin/sh

set -eu

validate_boolean() {
    case "$2" in
        true | false)
            ;;
        *)
            echo "Invalid value for $1: $2" >&2
            exit 1
            ;;
    esac
}

validate_boolean dangerously_insecure "${RECIPE_PARAM_DANGEROUSLY_INSECURE}"
validate_boolean factory_reset "${RECIPE_PARAM_FACTORY_RESET}"
validate_boolean system_commit "${RECIPE_PARAM_SYSTEM_COMMIT}"
validate_boolean system_reboot "${RECIPE_PARAM_SYSTEM_REBOOT}"
validate_boolean app_lifecycle "${RECIPE_PARAM_APP_LIFECYCLE}"

if ! command -v systemctl >/dev/null 2>&1; then
    echo "The Rugix Ctrl daemon recipe requires systemd." >&2
    exit 1
fi

if ! rugix-ctrl daemon --help >/dev/null 2>&1; then
    echo "The installed Rugix Ctrl release does not provide daemon mode." >&2
    exit 1
fi

if ! grep -q '^rugix-daemon:' /etc/group; then
    if command -v groupadd >/dev/null 2>&1; then
        groupadd --system rugix-daemon
    elif command -v addgroup >/dev/null 2>&1; then
        addgroup -S rugix-daemon
    else
        echo "Unable to create the rugix-daemon system group." >&2
        exit 1
    fi
fi

install -d -m 0755 /etc/rugix
cat >/etc/rugix/daemon.toml <<EOF
dangerously-insecure = ${RECIPE_PARAM_DANGEROUSLY_INSECURE}

[features]
factory-reset = ${RECIPE_PARAM_FACTORY_RESET}
system-commit = ${RECIPE_PARAM_SYSTEM_COMMIT}
system-reboot = ${RECIPE_PARAM_SYSTEM_REBOOT}
app-lifecycle = ${RECIPE_PARAM_APP_LIFECYCLE}
EOF
chmod 0644 /etc/rugix/daemon.toml

install -D -m 0644 \
    "${RECIPE_DIR}/files/rugix-ctrl-daemon.service" \
    -t /usr/lib/systemd/system/

systemctl enable rugix-ctrl-daemon.service
