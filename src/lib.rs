use std::net::Ipv6Addr;

/// Defangs a string by making URLs, IPs, domains, and emails inert.
///
/// Transformations applied:
/// - URL schemes: `https://` → `hxxps[://]`, `http://` → `hxxp[://]`, etc.
/// - Dots:        `.` → `[.]`
/// - At-sign:     `@` → `[@]` (emails)
/// - Colons:      `:` → `[:]` (IPv6)
pub fn defang(input: &str) -> String {
    let s = input.trim();

    if s.contains("://") {
        defang_url(s)
    } else if s.contains('@') {
        defang_email(s)
    } else if s.parse::<Ipv6Addr>().is_ok() {
        defang_ipv6(s)
    } else {
        // Covers IPv4, plain domains, and anything dot-separated.
        s.replace('.', "[.]")
    }
}

/// Reverses defanging, restoring the original string.
pub fn refang(input: &str) -> String {
    input
        // Scheme variants with bracketed separator
        .replace("hxxps[://]", "https://")
        .replace("hxxp[://]", "http://")
        .replace("fxxps[://]", "ftps://")
        .replace("fxxp[://]", "ftp://")
        // Scheme variants without bracketed separator (alternative form)
        .replace("hxxps://", "https://")
        .replace("hxxp://", "http://")
        .replace("fxxps://", "ftps://")
        .replace("fxxp://", "ftp://")
        // Bracketed separator alone
        .replace("[://]", "://")
        .replace("[@]", "@")
        .replace("[.]", ".")
        .replace("[:]", ":")
}

fn defang_url(s: &str) -> String {
    s.replace("https://", "hxxps[://]")
        .replace("http://", "hxxp[://]")
        .replace("ftps://", "fxxps[://]")
        .replace("ftp://", "fxxp[://]")
        .replace('.', "[.]")
}

fn defang_email(s: &str) -> String {
    s.replace('@', "[@]").replace('.', "[.]")
}

fn defang_ipv6(s: &str) -> String {
    s.replace(':', "[:]")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- defang ---

    #[test]
    fn defang_https_url() {
        assert_eq!(
            defang("https://www.example.com/path?q=1"),
            "hxxps[://]www[.]example[.]com/path?q=1"
        );
    }

    #[test]
    fn defang_http_url() {
        assert_eq!(
            defang("http://example.com"),
            "hxxp[://]example[.]com"
        );
    }

    #[test]
    fn defang_ftp_url() {
        assert_eq!(
            defang("ftp://files.example.com/pub"),
            "fxxp[://]files[.]example[.]com/pub"
        );
    }

    #[test]
    fn defang_domain() {
        assert_eq!(defang("example.com"), "example[.]com");
    }

    #[test]
    fn defang_subdomain() {
        assert_eq!(defang("sub.example.co.uk"), "sub[.]example[.]co[.]uk");
    }

    #[test]
    fn defang_ipv4() {
        assert_eq!(defang("192.168.1.1"), "192[.]168[.]1[.]1");
    }

    #[test]
    fn defang_ipv6() {
        assert_eq!(defang("2001:db8::1"), "2001[:]db8[:][:]1");
    }

    #[test]
    fn defang_ipv6_full() {
        assert_eq!(
            defang("2001:0db8:0000:0000:0000:0000:0000:0001"),
            "2001[:]0db8[:]0000[:]0000[:]0000[:]0000[:]0000[:]0001"
        );
    }

    #[test]
    fn defang_email() {
        assert_eq!(
            defang("user@example.com"),
            "user[@]example[.]com"
        );
    }

    #[test]
    fn defang_email_subdomain() {
        assert_eq!(
            defang("user.name@mail.example.org"),
            "user[.]name[@]mail[.]example[.]org"
        );
    }

    #[test]
    fn defang_empty() {
        assert_eq!(defang(""), "");
    }

    #[test]
    fn defang_trims_whitespace() {
        assert_eq!(defang("  example.com  "), "example[.]com");
    }

    // --- refang ---

    #[test]
    fn refang_https_url() {
        assert_eq!(
            refang("hxxps[://]www[.]example[.]com/path?q=1"),
            "https://www.example.com/path?q=1"
        );
    }

    #[test]
    fn refang_http_url() {
        assert_eq!(refang("hxxp[://]example[.]com"), "http://example.com");
    }

    #[test]
    fn refang_http_url_no_bracket() {
        assert_eq!(refang("hxxp://example[.]com"), "http://example.com");
    }

    #[test]
    fn refang_ftp_url() {
        assert_eq!(
            refang("fxxp[://]files[.]example[.]com/pub"),
            "ftp://files.example.com/pub"
        );
    }

    #[test]
    fn refang_domain() {
        assert_eq!(refang("example[.]com"), "example.com");
    }

    #[test]
    fn refang_ipv4() {
        assert_eq!(refang("192[.]168[.]1[.]1"), "192.168.1.1");
    }

    #[test]
    fn refang_ipv6() {
        assert_eq!(refang("2001[:]db8[:][:]1"), "2001:db8::1");
    }

    #[test]
    fn refang_email() {
        assert_eq!(refang("user[@]example[.]com"), "user@example.com");
    }

    #[test]
    fn refang_empty() {
        assert_eq!(refang(""), "");
    }

    // --- round-trip ---

    #[test]
    fn roundtrip_https_url() {
        let original = "https://www.example.com/path";
        assert_eq!(refang(&defang(original)), original);
    }

    #[test]
    fn roundtrip_ipv4() {
        let original = "10.0.0.1";
        assert_eq!(refang(&defang(original)), original);
    }

    #[test]
    fn roundtrip_ipv6() {
        let original = "2001:db8::1";
        assert_eq!(refang(&defang(original)), original);
    }

    #[test]
    fn roundtrip_email() {
        let original = "admin@example.org";
        assert_eq!(refang(&defang(original)), original);
    }

    #[test]
    fn roundtrip_domain() {
        let original = "sub.example.co.uk";
        assert_eq!(refang(&defang(original)), original);
    }
}
