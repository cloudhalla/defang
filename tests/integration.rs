// Integration tests exercise the public API from the outside.

use defang::{defang, refang};

#[test]
fn defang_common_ioc_types() {
    assert_eq!(defang("https://malware.example.com"), "hxxps[://]malware[.]example[.]com");
    assert_eq!(defang("http://evil.io/payload.exe"), "hxxp[://]evil[.]io/payload[.]exe");
    assert_eq!(defang("ftp://drop.zone/file"), "fxxp[://]drop[.]zone/file");
    assert_eq!(defang("attacker@phish.net"), "attacker[@]phish[.]net");
    assert_eq!(defang("10.20.30.40"), "10[.]20[.]30[.]40");
    assert_eq!(defang("fe80::1%eth0"), "fe80::1%eth0"); // zone-ID suffix prevents IPv6 parse; returned as-is
    assert_eq!(defang("::1"), "[:][:]1");
}

#[test]
fn refang_common_defanged_forms() {
    assert_eq!(refang("hxxps[://]example[.]com"), "https://example.com");
    assert_eq!(refang("hxxp[://]example[.]com"), "http://example.com");
    assert_eq!(refang("hxxp://example[.]com"), "http://example.com");
    assert_eq!(refang("user[@]host[.]org"), "user@host.org");
    assert_eq!(refang("192[.]0[.]2[.]1"), "192.0.2.1");
    assert_eq!(refang("2001[:]db8[:][:]1"), "2001:db8::1");
}

#[test]
fn roundtrips_are_identity() {
    let cases = [
        "https://www.example.com/path?q=foo&bar=1",
        "http://sub.domain.co.uk",
        "ftp://files.host.net/archive.tar.gz",
        "user@example.com",
        "first.last@mail.example.org",
        "192.168.100.200",
        "172.16.0.1",
        "2001:db8::",
        "::1",
        "2001:0db8:0000:0000:0000:0000:0000:0001",
        "example.com",
        "sub.example.co.uk",
    ];

    for &original in &cases {
        let result = refang(&defang(original));
        assert_eq!(result, original, "round-trip failed for: {original}");
    }
}
