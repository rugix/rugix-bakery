# Rugix U-Boot boot script for A/B system updates.
#
# Board-independent: every board-dependent value is resolved from U-Boot
# environment variables that the vendor's u-boot populates at startup or
# that distroboot sets while scanning for this script:
#
#   ${devnum}       MMC device number set by distroboot before sourcing.
#   ${console}      Serial console set by U-Boot defconfig.
#   ${fdtfile}      Device tree blob filename set by U-Boot defconfig.
#
# State (rugix_bootpart, rugix_boot_spare) lives in a binary U-Boot env
# file (rugix.env) on the FAT config partition. Userspace fw_printenv /
# fw_setenv read the same file via /etc/fw_env.config.

echo "Starting boot..."

mmc dev ${devnum}
mmc rescan

# Load persisted A/B state from the config partition.
if load mmc ${devnum}:1 ${loadaddr} rugix.env; then
  env import -c ${loadaddr} ${filesize}
else
  setenv rugix_state_dirty 1
fi

# First-boot defaults.
if test -z "${rugix_bootpart}"; then
  setenv rugix_boot_spare 0
  setenv rugix_bootpart 2
  setenv rugix_state_dirty 1
fi

echo "Boot Spare: " ${rugix_boot_spare}
echo "Bootpart: " ${rugix_bootpart}

# Determine which system partition to boot.
if test "${rugix_boot_spare}" = "1"; then
  if test "${rugix_bootpart}" = "3"; then
    setenv rugix_boot_part 2
  else
    setenv rugix_boot_part 3
  fi
  setenv rugix_boot_spare 0
  setenv rugix_state_dirty 1
else
  if test "${rugix_bootpart}" = "3"; then
    setenv rugix_boot_part 3
  else
    setenv rugix_boot_part 2
  fi
fi

echo "Bootdev: mmc ${devnum}:" ${rugix_boot_part}

# Persist only when state actually changed.
if test -n "${rugix_state_dirty}"; then
  env export -c -s 0x4000 ${loadaddr} rugix_bootpart rugix_boot_spare
  fatwrite mmc ${devnum}:1 ${loadaddr} rugix.env 0x4000
fi

# Resolve PARTUUID so root= doesn't depend on device enumeration order.
part uuid mmc ${devnum}:${rugix_boot_part} rugix_root_uuid

setenv bootargs "root=PARTUUID=${rugix_root_uuid} rootwait init=/usr/bin/rugix-ctrl ro console=${console} panic=60"

# Load kernel and device tree from the selected system partition's /boot.
load mmc ${devnum}:${rugix_boot_part} ${kernel_addr_r} boot/Image
load mmc ${devnum}:${rugix_boot_part} ${fdt_addr_r} boot/${fdtfile}

booti ${kernel_addr_r} - ${fdt_addr_r}
