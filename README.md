# defang

A small CLI utility to **defang** and **refang** indicators of compromise (IoCs) so they can be shared safely by email or chat without creating accidental hyperlinks.

## What it handles

| Type   | Input                    | Defanged                       |
|--------|--------------------------|--------------------------------|
| HTTPS  | `https://evil.example.com` | `hxxps[://]evil[.]example[.]com` |
| HTTP   | `http://x.io/payload`    | `hxxp[://]x[.]io/payload`      |
| FTP    | `ftp://files.net/file`   | `fxxp[://]files[.]net/file`    |
| Email  | `user@phish.net`         | `user[@]phish[.]net`           |
| IPv4   | `192.168.1.1`            | `192[.]168[.]1[.]1`            |
| IPv6   | `2001:db8::1`            | `2001[:]db8[:][:]1`            |
| Domain | `sub.example.co.uk`      | `sub[.]example[.]co[.]uk`      |

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Defang one or more arguments
defang https://malware.example.com 192.168.1.1 user@phish.net

# Refang previously defanged strings
defang --refang "hxxps[://]malware[.]example[.]com" "192[.]168[.]1[.]1"

# Pipe from stdin
echo "evil.example.com" | defang
cat iocs.txt | defang
cat defanged.txt | defang --refang
```

## License

MIT
