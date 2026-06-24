# GoXLR Drivers (Windows)

This folder holds the **official TC-Helicon GoXLR Windows driver** package.

above-goxlr does **not** include or build a driver. On Windows the utility loads
the closed-source TC-Helicon driver DLL at runtime (resolved from the registry,
defaulting to `C:\Program Files\TC-HELICON\GoXLR_Audio_Driver\W10_x64\goxlr_audioapi_x64.dll`).
On Linux/macOS no proprietary driver is required (direct USB access).

Because TC-Helicon has wound down distribution, this folder keeps a known-good
copy so the driver can always be reinstalled, independent of any external mirror.

## What to put here

Drop the driver zip in this folder, e.g.:

```
drivers/TC-Helicon_GoXLR_Driver_5.57.zip
```

Known-good version: **5.57.0.26390** (signed 2023-05-22 by Microsoft Windows
Hardware Compatibility Publisher). Upstream mirror:
<https://utility.frostycoolslug.com/update-site/drivers/TC-Helicon_GoXLR_Driver_5.57.zip>

## Installing

Unzip and run the installer, or install the `.inf` packages directly:

```powershell
pnputil /add-driver goxlr_audio.inf /install
pnputil /add-driver goxlr_audioks.inf /install
```

## License note

The driver is proprietary TC-Helicon software and is **not** covered by this
project's license. It is kept here only for archival / reinstall convenience.
