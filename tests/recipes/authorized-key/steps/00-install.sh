#!/bin/bash

set -euo pipefail

install -d -m 700 /root/.ssh
cat "${RUGIX_PROJECT_DIR}/.rugix-testkit/id_rsa.pub" >>/root/.ssh/authorized_keys
