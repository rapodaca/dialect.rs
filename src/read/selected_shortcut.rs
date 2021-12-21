use lyn::Scanner;

use crate::graph::SelectedShortcut;

pub fn selected_shortcut(scanner: &mut Scanner) -> Option<SelectedShortcut> {
    scanner.transform(|character| match character {
        'b' => Some(SelectedShortcut::B),
        'c' => Some(SelectedShortcut::C),
        'n' => Some(SelectedShortcut::N),
        'o' => Some(SelectedShortcut::O),
        'p' => Some(SelectedShortcut::P),
        's' => Some(SelectedShortcut::S),
        _ => None,
    })
}
