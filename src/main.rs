mod editor;
use editor::Editor;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut editor;
    if let Some(filename) = args.get(1) {
        // if args[1] is given, it should be filename to open
        editor = Editor::new(filename);
    } else {
        editor = Editor::default();
    }
    editor.run();
}
