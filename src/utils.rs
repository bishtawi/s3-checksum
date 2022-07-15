pub fn display_bytes(size: i64) -> String {
    if size < 1000 {
        format!("{} bytes", size)
    } else if size < 1_000_000 {
        format!("{}.{:0>2} kilobytes", size / 1000, (size % 1000) / 10)
    } else if size < 1_000_000_000 {
        format!(
            "{}.{:0>2} megabytes",
            size / 1_000_000,
            (size % 1_000_000) / 10_000
        )
    } else if size < 1_000_000_000_000 {
        format!(
            "{}.{:0>2} gigabytes",
            size / 1_000_000_000,
            (size % 1_000_000_000) / 10_000_000
        )
    } else {
        format!(
            "{}.{:0>2} terrabytes",
            size / 1_000_000_000_000,
            (size % 1_000_000_000_000) / 10_000_000_000
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_bytes() {
        assert_eq!(display_bytes(0), "0 bytes");
        assert_eq!(display_bytes(999), "999 bytes");
        assert_eq!(display_bytes(1000), "1.00 kilobytes");
        assert_eq!(display_bytes(1001), "1.00 kilobytes");
        assert_eq!(display_bytes(1010), "1.01 kilobytes");
        assert_eq!(display_bytes(1100), "1.10 kilobytes");
        assert_eq!(display_bytes(1999), "1.99 kilobytes");
        assert_eq!(display_bytes(999_999), "999.99 kilobytes");
        assert_eq!(display_bytes(1_000_000), "1.00 megabytes");
        assert_eq!(display_bytes(1_000_001), "1.00 megabytes");
        assert_eq!(display_bytes(1_010_000), "1.01 megabytes");
        assert_eq!(display_bytes(1_100_000), "1.10 megabytes");
        assert_eq!(display_bytes(1_999_999), "1.99 megabytes");
        assert_eq!(display_bytes(999_999_999), "999.99 megabytes");
        assert_eq!(display_bytes(1_000_000_000), "1.00 gigabytes");
        assert_eq!(display_bytes(1_000_000_001), "1.00 gigabytes");
        assert_eq!(display_bytes(1_010_000_000), "1.01 gigabytes");
        assert_eq!(display_bytes(1_100_000_000), "1.10 gigabytes");
        assert_eq!(display_bytes(1_999_999_999), "1.99 gigabytes");
        assert_eq!(display_bytes(999_999_999_999), "999.99 gigabytes");
        assert_eq!(display_bytes(1_000_000_000_000), "1.00 terrabytes");
        assert_eq!(display_bytes(1_000_000_000_001), "1.00 terrabytes");
        assert_eq!(display_bytes(1_010_000_000_000), "1.01 terrabytes");
        assert_eq!(display_bytes(1_100_000_000_000), "1.10 terrabytes");
        assert_eq!(display_bytes(1_999_999_999_999), "1.99 terrabytes");
        assert_eq!(display_bytes(1_000_000_000_000_000), "1000.00 terrabytes");
        assert_eq!(display_bytes(1_000_500_000_000_000), "1000.50 terrabytes");
    }
}
