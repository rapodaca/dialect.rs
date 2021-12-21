use lyn::Scanner;

use super::non_zero;

pub fn digit(scanner: &mut Scanner) -> Option<u8> {
    if scanner.take(&'0') {
        Some(0)
    } else {
        non_zero(scanner)
    }
}
