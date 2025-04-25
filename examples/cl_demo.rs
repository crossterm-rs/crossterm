use std::io;

use calloop::EventLoop;
use crossterm::{
    event::{
        runtime::calloop::UnixInternalEventSource, DisableBracketedPaste, DisableFocusChange,
        DisableMouseCapture, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture,
        KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};

struct LoopData {
    exit: bool,
}

fn event_loop() {
    let mut el = EventLoop::try_new().unwrap();
    el.handle()
        .insert_source(
            UnixInternalEventSource::new().unwrap(),
            |es, _, data: &mut LoopData| {
                println!("{:?}\r", es);
                es.iter().for_each(|e| {
                    if let crossterm::event::InternalEvent::Event(crossterm::event::Event::Key(
                        key_event,
                    )) = e
                    {
                        if key_event.code.is_esc() {
                            data.exit = true;
                        }
                    }
                });
                Ok(())
            },
        )
        .unwrap();

    let mut a = LoopData { exit: false };
    loop {
        el.dispatch(None, &mut a).unwrap();
        if a.exit {
            break;
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();

    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );

    if supports_keyboard_enhancement {
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )
        .unwrap();
    }

    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
    )
    .unwrap();

    event_loop();

    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags).unwrap();
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )
    .unwrap();

    disable_raw_mode().unwrap();
}
