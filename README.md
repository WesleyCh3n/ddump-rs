# ddump

Dumping the device info using windows abi by rust.

## How to Build

Below requirement are tested on windows 10.

Using rust `1.62.0` with `stable-x86_64-pc-windows-msvc` toolchain.

```bash
cargo build --release
```

## Usage

```bash
$ ddump
# or
$ ddump <some_name.json>
```

It will create `device_info.json` when no filename specify.

Output json file will something like this

```json
[
  {
    "class": "System",
    "enumerator": "ACPI",
    "description": "Motherboard resources",
    "manufacturer": "(Standard system devices)",
    "hardware_id": "ACPI\\VEN_PNP&DEV_0C02",
    "compatible_id": "",
    "class_guid": "{4d36e97d-e325-11ce-bfc1-08002be10318}"
  },
  {
    "class": "System",
    "enumerator": "ACPI",
    "description": "Motherboard resources",
    "manufacturer": "(Standard system devices)",
    "hardware_id": "ACPI\\VEN_PNP&DEV_0C02",
    "compatible_id": "",
    "class_guid": "{4d36e97d-e325-11ce-bfc1-08002be10318}"
  },
...
```
