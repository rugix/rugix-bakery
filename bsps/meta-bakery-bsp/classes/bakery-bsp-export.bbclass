# Export BSP metadata consumed by the rugix_bsp Python tooling.
#
# Inherit this class in an image recipe (or add to IMAGE_CLASSES) to
# emit a bakery-bsp-metadata.json alongside the deployed images. The
# Python tooling can read this file to pick up board-specific values
# that are hard to determine statically (e.g., IMX_BOOT_SEEK varies
# by i.MX sub-family).

python do_bakery_bsp_export() {
    import json, os

    metadata = {
        "machine": d.getVar("MACHINE"),
        "arch": d.getVar("TUNE_ARCH"),
        "kernel_imagetype": d.getVar("KERNEL_IMAGETYPE"),
        "image_boot_files": (d.getVar("IMAGE_BOOT_FILES") or "").split(),
        # NXP i.MX specific.
        "imx_boot_seek": d.getVar("IMX_BOOT_SEEK"),
        # Rockchip specific.
        "spl_binary": d.getVar("SPL_BINARY"),
        "uboot_suffix": d.getVar("UBOOT_SUFFIX"),
    }

    # Strip None values for cleaner output.
    metadata = {k: v for k, v in metadata.items() if v is not None}

    deploy = d.getVar("IMGDEPLOYDIR")
    path = os.path.join(deploy, "bakery-bsp-metadata.json")
    with open(path, "w") as f:
        json.dump(metadata, f, indent=2)
    bb.note("bakery-bsp-export: wrote %s" % path)
}

addtask bakery_bsp_export after do_image_complete before do_build
