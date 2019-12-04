use std::io::prelude::*;

enum State {
    Normal,
    Escape,
    Csi,
}

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    let mut state = State::Normal;

    for b in reader.bytes() {
        let b = b?;
        match &state {
            State::Normal => {
                if b == 0x1B { // ESC
                    state = State::Escape;
                } else {
                    writer.write(&[b])?;
                }
            },
            State::Escape => {
                if b == 0x5B { // [
                    state = State::Csi;
                } else {
                    state = State::Normal;
                }
            },
            State::Csi => {
                if b >= 0x40 && b < 0x80 {
                    state = State::Normal;
                }
            },
        }
    }
    Ok(())
}
