//! Guitar fretboard diagram — minimalist black & white with blue dots.

use hagoromo::*;

// ── Design tokens ─────────────────────────────────────────────────────────────

const VS: f64 = 22.0;              // vertical spacing between frets
const HS: f64 = 16.0;              // horizontal spacing between strings
const DOT_R: f64 = 4.5;            // note dot radius

const LINE_W: f64 = 0.8;           // fret / string line width
const NUT_W: f64 = 3.0;            // nut bar line width
const BORDER_W: f64 = 1.2;         // outer frame line width

const GRID_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);  // dark gray lines
const DOT_BLUE: Color   = Color::rgb(0.22, 0.44, 0.85);  // note dot fill
const ROOT_BLUE: Color  = Color::rgb(0.08, 0.25, 0.65);  // root note (darker)
const BG: Color         = Color::rgb(1.00, 1.00, 1.00);  // white background
const LABEL: Color      = Color::rgb(0.20, 0.20, 0.20);  // text

// ── Empty fretboard ───────────────────────────────────────────────────────────

fn empty_board(n_frets: usize, n_strings: usize, start_fret: usize) -> Diagram {
    let w = (n_strings - 1) as f64 * HS;
    let h = n_frets as f64 * VS;

    // White background with a clean border
    let bg = rect(w, h)
        .fc(BG)
        .lc(GRID_COLOR)
        .stroke_width(BORDER_W)
        .translate(w / 2.0, h / 2.0);

    // Nut: thick solid bar, or dashed offset marker
    let nut = if start_fret == 0 {
        stroke_trail(hrule(w)).lc(GRID_COLOR).lw(NUT_W)
    } else {
        stroke_trail(hrule(w)).lc(GRID_COLOR).lw(LINE_W)
    };

    // Fret lines
    let frets: Diagram = (1..n_frets)
        .map(|i| {
            stroke_trail(hrule(w))
                .lc(GRID_COLOR)
                .lw(LINE_W)
                .translate_y(i as f64 * VS)
        })
        .fold(Diagram::empty(), |acc, d| acc + d);

    // String lines
    let strings: Diagram = (0..n_strings)
        .map(|i| {
            stroke_trail(vrule(h))
                .lc(GRID_COLOR)
                .lw(LINE_W * 0.8)
                .translate_x(i as f64 * HS)
        })
        .fold(Diagram::empty(), |acc, d| acc + d);

    bg + nut + frets + strings
}

// ── Dot marker ────────────────────────────────────────────────────────────────

fn fret_dot(string_idx: usize, fret_offset: usize, is_root: bool) -> Diagram {
    let x = string_idx as f64 * HS;
    let y = (fret_offset as f64 + 0.5) * VS;
    let color = if is_root { ROOT_BLUE } else { DOT_BLUE };
    circle(DOT_R)
        .fc(color)
        .stroke_width(0.0)
        .translate(x, y)
}

// ── Labels ────────────────────────────────────────────────────────────────────

fn string_labels(names: &[&str], n_frets: usize) -> Diagram {
    let h = n_frets as f64 * VS;
    names
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            text(name, 6.5)
                .fc(LABEL)
                .translate(i as f64 * HS, h + VS * 0.65)
        })
        .fold(Diagram::empty(), |acc, d| acc + d)
}

fn position_label(start_fret: usize) -> Diagram {
    text(format!("{}fr", start_fret), 6.5)
        .fc(LABEL)
        .translate(-14.0, VS * 0.5)
}

fn title(label: &str, n_strings: usize) -> Diagram {
    let w = (n_strings - 1) as f64 * HS;
    text(label, 8.0)
        .fc(LABEL)
        .bold()
        .translate(w / 2.0, -VS * 0.6)
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let n_strings = 6;
    let n_frets = 4;
    let start_fret = 5;

    // A minor pentatonic "box 1" at the 5th position.
    // (string_index, fret_offset, is_root)
    // Strings: low E=0 … high e=5.  Root = A.
    let dots: &[(usize, usize, bool)] = &[
        (0, 0, true),  // E  fret 5 → A ●
        (0, 3, false), // E  fret 8 → C
        (1, 0, false), // A  fret 5 → D
        (1, 2, false), // A  fret 7 → E
        (2, 0, false), // D  fret 5 → G
        (2, 2, true),  // D  fret 7 → A ●
        (3, 0, false), // G  fret 5 → C
        (3, 2, false), // G  fret 7 → D
        (4, 0, false), // B  fret 5 → E
        (4, 3, false), // B  fret 8 → G
        (5, 0, true),  // e  fret 5 → A ●
        (5, 3, false), // e  fret 8 → C
    ];

    let board = empty_board(n_frets, n_strings, start_fret);

    let dot_layer: Diagram = dots
        .iter()
        .map(|&(s, f, root)| fret_dot(s, f, root))
        .fold(Diagram::empty(), |acc, d| acc + d);

    let diagram = board
        + dot_layer
        + string_labels(&["E", "A", "D", "G", "B", "e"], n_frets)
        + position_label(start_fret)
        + title("Am Pentatonic", n_strings);

    let opts = RenderOptions { padding: 14.0, ..Default::default() };
    let svg = render_svg(&diagram, &opts);

    std::fs::write("fretboard.svg", &svg).unwrap();
    eprintln!("Written to fretboard.svg");
}
