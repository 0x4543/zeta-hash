# ZetaHash

Lightweight CLI and library for hashing strings, files, and generating random salts with multiple algorithms.

## Features
- SHA256, Keccak256, Blake3 for strings
- SHA256, Keccak256, Blake3 for files
- Generate random salts of specified length

## Usage

### Hash a string
```bash
zeta-hash sha256 "hello world"
zeta-hash keccak256 "hello world"
zeta-hash blake3 "hello world"
```

### Hash a file
```bash
zeta-hash file path/to/file.txt sha256
zeta-hash file path/to/file.txt keccak256
zeta-hash file path/to/file.txt blake3
```

### Generate a random salt
```bash
zeta-hash salt 16  # generates a 16-character random salt
```

## Example Output
```
zeta-hash sha256 "hello"
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

## License
MIT

