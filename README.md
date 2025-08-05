# peeper-rs

Example of usage of the [CVE-2023-36266](https://nvd.nist.gov/vuln/detail/CVE-2023-36266) (won't fix) to extract credentials from [Keeper](https://www.keepersecurity.com/).

# Usage
```powershell
PS Z:\> .\peeper.exe --help
A tool to extract keeper credentials from chromium and desktop

Usage: peeper.exe [OPTIONS] <APPLICATION>

Arguments:
  <APPLICATION>  [possible values: msedge, chrome, desktop]

Options:
  -v, --verbose  Enable verbose logging
  -h, --help     Print help
  -V, --version  Print version
PS Z:\>
```

# Compiling

## Without debug info
```bash
$ make
```

## With debug info
```bash
$ make debug
```
