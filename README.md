# 🦀 MSICrab

![License](https://img.shields.io/badge/license-GPLv3.0-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Rust](https://img.shields.io/badge/Rust-%E2%9C%A8-orange)

**MSICrab** is a Rust-based tool that lists and interacts with installed MSI files on a Windows system. It is designed for pentesters and researchers to facilitate **Local Privilege Escalation** analysis. It is mostly meant to be run "on site" during a pentest, since I wanted a quick and easy method of repairing all installed MSI files for LPE.

## Features

- Retrieve and export a list of all installed MSI files on the system
- Copy all MSIs to a folder for easier analysis 
- Filter for default vendors
- Automatically run repair process
- Written in Rust

## Building

### Prerequisites
- **Windows 10/11**
- **[Rust](https://www.rust-lang.org/tools/install)** 

Clone the repository and build the binary:

```sh
git clone https://github.com/pongratzp/msicrab.git
cd msicrab
cargo build --release
```

The compiled binary will be in `target/release/msicrab.exe`.

---

## Usage

Run MSICrab to list all installed MSI files:

```sh
.\msicrab.exe
```

### Advanced Options

| Option | Description |
|--------|-------------|
| `--filter` | Filter for default vendors |
| `--csv` | Writes the list of installed MSIs to a .csv file |
| `--repair` | Automatically repair MSIs |
| `--extract` | Copy MSIs to output folder for documentation purposes |
| `--procmon` | Spawns Procmon64.exe for LPE analysis (needs admin rights and procmon) |
| `--help` | Display usage information |

### Procmon information
Please download [Procmon64.exe](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon) yourself and drop it into the same folder as msicrab.exe

### Example:

Analysis on a live system (low priv pentest mode):
```sh
msicrab.exe --filter --repair
```

Analysis on a live system (high priv pentest mode):
```sh
msicrab.exe --filter --csv --repair --procmon
```

Copy MSIs for analysis on a remote system:
```sh
msicrab.exe --csv --extract
```

## References & Inspiration

This project was mostly inspired by [Mandiant's research on this topic](https://cloud.google.com/blog/topics/threat-intelligence/privileges-third-party-windows-installers/?hl=en) and their tool [https://github.com/mandiant/msi-search](https://github.com/mandiant/msi-search) as well as [this really nice writeup](https://badoption.eu/blog/2023/10/03/MSIFortune.html).

Other tools that may help you in discovering and exploiting MSI vulnerabilities:

- SetOpLock.exe - [https://github.com/googleprojectzero/symboliclink-testing-tools](https://github.com/googleprojectzero/symboliclink-testing-tools)
- msiscan from SecConsult - [https://github.com/sec-consult/msiscan](https://github.com/sec-consult/msiscan)


## License

This project is licensed under the **GPLv3.0 License** – see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! This project is a WIP (as most pentest tools are anyway) so keep that in mind.
