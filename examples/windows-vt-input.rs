//! Windows VT-input diagnostic example.
//!
//! This example constructs crossterm's event reader before raw mode is enabled, then
//! enables raw mode and reads events with that same source. The mode is queried through
//! the same `CONIN$` handle type used by crossterm, so this exercises mode changes after
//! source construction and verifies that event parsing does not rely on a lifetime VT
//! cache. The diagnostic also prints the original and final VT-input bits.
//!
//! On Windows, run with:
//!
//! ```text
//! cargo run --example windows-vt-input --all-features
//! ```
//!
//! While it is running, phase 1 checks the original input transport (press and
//! release `P` to begin phase 2 when VT was initially off; with pre-existing VT
//! input, pressing `P` is sufficient), then phase 2 checks bracketed paste, SGR mouse, and a physical
//! numeric-keypad Alt+numpad character. Every event is printed with `Debug`,
//! including `KeyEventKind` and bracketed-paste events. Press Escape to finish.
//!
//! Expected observations on conhost include `a` and F1 producing both Press and Release
//! events (held keys may be exposed as additional Press records) in phase 1,
//! Alt+numpad 0233 producing one character
//! Press in phase 2, and a multiline paste producing one `Paste` event rather than one
//! event per line. Windows Terminal/ConPTY can be Press-only during phase 2 because
//! VT input does not preserve all key-release records.

#[cfg(all(any(windows, test), feature = "events"))]
#[cfg_attr(not(windows), allow(dead_code))]
mod stats {
    use std::collections::{HashMap, HashSet};

    use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

    #[derive(Default)]
    pub(super) struct SessionStats {
        mouse_moved: u64,
        mouse_clicks: u64,
        mouse_drags: u64,
        mouse_wheels: u64,
        key_presses: u64,
        key_releases: u64,
        repeated_presses_while_held: u64,
        repeat_events: u64,
        held_keys: HashSet<(KeyCode, KeyModifiers)>,
        press_balance: HashMap<(KeyCode, KeyModifiers), i32>,
        duplicate_candidates: u64,
        last_alt_press: Option<(KeyCode, KeyModifiers)>,
        duplicate_candidate: Option<(KeyCode, KeyModifiers)>,
    }

    impl SessionStats {
        pub(super) fn clear_duplicate_state(&mut self) {
            self.last_alt_press = None;
            self.duplicate_candidate = None;
        }

        pub(super) fn cancel_control_press(&mut self, event: &Event) {
            if let Event::Key(key) = event
                && key.kind == KeyEventKind::Press
            {
                let identity = (key.code, key.modifiers);
                if self.held_keys.remove(&identity) {
                    *self.press_balance.entry(identity).or_default() -= 1;
                }
            }
        }

        pub(super) fn record(&mut self, event: &Event) {
            match event {
                Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Moved) => {
                    self.mouse_moved += 1;
                    self.clear_duplicate_state();
                }
                Event::Mouse(mouse) => {
                    self.clear_duplicate_state();
                    match mouse.kind {
                        MouseEventKind::Down(_) | MouseEventKind::Up(_) => self.mouse_clicks += 1,
                        MouseEventKind::Drag(_) => self.mouse_drags += 1,
                        MouseEventKind::ScrollUp
                        | MouseEventKind::ScrollDown
                        | MouseEventKind::ScrollLeft
                        | MouseEventKind::ScrollRight => self.mouse_wheels += 1,
                        MouseEventKind::Moved => unreachable!(),
                    }
                }
                Event::Key(key) if key.code == KeyCode::Esc => self.clear_duplicate_state(),
                Event::Key(key) => match key.kind {
                    KeyEventKind::Press => {
                        self.key_presses += 1;
                        let identity = (key.code, key.modifiers);
                        if self.duplicate_candidate == Some(identity) {
                            self.duplicate_candidates += 1;
                        }
                        self.duplicate_candidate = None;
                        if key.modifiers.contains(KeyModifiers::ALT) {
                            self.last_alt_press = Some(identity);
                        } else {
                            self.last_alt_press = None;
                        }
                        if !self.held_keys.insert(identity) {
                            self.repeated_presses_while_held += 1;
                        } else {
                            *self.press_balance.entry(identity).or_default() += 1;
                        }
                    }
                    KeyEventKind::Release => {
                        self.key_releases += 1;
                        let identity = (key.code, key.modifiers);
                        // Removing a held identity balances its first Press. For a release-first
                        // or extra Release, decrementing a new entry keeps the negative mismatch.
                        self.held_keys.remove(&identity);
                        *self.press_balance.entry(identity).or_default() -= 1;
                        self.duplicate_candidate = self
                            .last_alt_press
                            .filter(|(press_code, press_modifiers)| {
                                *press_code == key.code && *press_modifiers == key.modifiers
                            })
                            .map(|(code, _)| (code, KeyModifiers::NONE));
                        self.last_alt_press = None;
                    }
                    KeyEventKind::Repeat => {
                        self.repeat_events += 1;
                        self.clear_duplicate_state();
                    }
                },
                _ => {
                    self.clear_duplicate_state();
                }
            }
        }

        fn event_imbalance(&self) -> i32 {
            self.press_balance.values().map(|count| count.abs()).sum()
        }

        pub(super) fn print_summary(&self) {
            let mismatches = self
                .press_balance
                .values()
                .filter(|count| **count != 0)
                .count();
            println!("\nSummary:");
            println!(
                "  key Press={}, Release={}",
                self.key_presses, self.key_releases
            );
            println!(
                "  additional Press while held={}, Repeat-kind events={}",
                self.repeated_presses_while_held, self.repeat_events
            );
            println!(
                "  mouse Moved={} (suppressed from log), click={}, drag={}, wheel={}",
                self.mouse_moved, self.mouse_clicks, self.mouse_drags, self.mouse_wheels
            );
            println!(
                "  Press/Release mismatches={mismatches}, event imbalance={}",
                self.event_imbalance()
            );
            println!(
                "  ALT Press -> matching Release -> same-code NONE Press duplicates={}",
                self.duplicate_candidates
            );
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn key(code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind) -> Event {
            Event::Key(crossterm::event::KeyEvent::new_with_kind(
                code, modifiers, kind,
            ))
        }

        #[test]
        fn held_additional_press_does_not_imbalance() {
            let mut stats = SessionStats::default();
            let event = key(KeyCode::Char('a'), KeyModifiers::NONE, KeyEventKind::Press);
            stats.record(&event);
            stats.record(&event);
            stats.record(&key(
                KeyCode::Char('a'),
                KeyModifiers::NONE,
                KeyEventKind::Release,
            ));
            assert_eq!(stats.repeated_presses_while_held, 1);
            assert_eq!(stats.event_imbalance(), 0);
        }

        #[test]
        fn release_first_reports_negative_imbalance() {
            let mut stats = SessionStats::default();
            stats.record(&key(
                KeyCode::Char('a'),
                KeyModifiers::NONE,
                KeyEventKind::Release,
            ));
            assert_eq!(stats.event_imbalance(), 1);
            assert_eq!(
                stats.press_balance[&(KeyCode::Char('a'), KeyModifiers::NONE)],
                -1
            );
        }

        #[test]
        fn modifier_mismatch_keeps_two_unbalanced_identities() {
            let mut stats = SessionStats::default();
            stats.record(&key(
                KeyCode::Char('B'),
                KeyModifiers::SHIFT | KeyModifiers::CONTROL,
                KeyEventKind::Press,
            ));
            stats.record(&key(
                KeyCode::Char('b'),
                KeyModifiers::NONE,
                KeyEventKind::Release,
            ));
            assert_eq!(stats.press_balance.len(), 2);
            assert_eq!(stats.event_imbalance(), 2);
        }

        #[test]
        fn cancel_control_press_balances_transition_event() {
            let mut stats = SessionStats::default();
            let event = key(KeyCode::Char('p'), KeyModifiers::NONE, KeyEventKind::Press);
            stats.record(&event);
            stats.cancel_control_press(&event);
            assert_eq!(stats.event_imbalance(), 0);
            assert!(stats.held_keys.is_empty());
        }
    }
}

#[cfg(windows)]
mod windows {
    use std::io;
    use std::time::Duration;

    use super::stats::SessionStats;
    use crossterm::event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
        Event, KeyCode, KeyEventKind, MouseEventKind, poll, read,
    };
    use crossterm::execute;
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use crossterm_winapi::{ConsoleMode, Handle};

    const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;
    const ENABLE_MOUSE_INPUT: u32 = 0x0010;
    const ENABLE_WINDOW_INPUT: u32 = 0x0008;
    const ENABLE_EXTENDED_FLAGS: u32 = 0x0080;

    fn current_console_mode() -> io::Result<u32> {
        let handle = Handle::current_in_handle()?;
        ConsoleMode::from(handle).mode()
    }

    fn print_mode(label: &str, mode: u32) {
        let vt_input = mode & ENABLE_VIRTUAL_TERMINAL_INPUT != 0;
        println!("{label}: mode=0x{mode:08x}, VT_INPUT={vt_input}");
    }

    fn is_transition_p(event: &Event) -> bool {
        matches!(
            event,
            Event::Key(key) if matches!(key.code, KeyCode::Char('p') | KeyCode::Char('P'))
        )
    }

    fn is_transition_p_release(event: &Event) -> bool {
        is_transition_p(event)
            && matches!(event, Event::Key(key) if key.kind == KeyEventKind::Release)
    }

    fn drain_startup_events(initially_available: bool) -> io::Result<()> {
        let mut available = initially_available;
        while available {
            let event = read()?;
            if !matches!(&event, Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Moved))
            {
                println!("Startup event: {event:?}");
            }
            // Poll only with a zero timeout: drain the records already queued after the
            // initial poll, then begin the interactive phase without waiting for new input.
            // A new record racing with the final zero-time poll belongs to the session.
            available = poll(Duration::ZERO)?;
        }
        Ok(())
    }

    fn read_phase(
        stats: &mut SessionStats,
        phase: &str,
        transition_on_press: bool,
        mut ignore_next_transition_release: bool,
    ) -> io::Result<(bool, bool)> {
        // A pre-existing VT transport may not emit the P release. In that case
        // transition on the press and ignore at most the immediately following
        // buffered release in phase 2.
        println!("{phase}");
        let mut phase_transition_pending = false;
        loop {
            let event = read()?;
            if !matches!(&event, Event::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Moved))
            {
                println!("Event: {event:?}");
            }
            // The phase gate applies only to transitioning out of Phase 1. The
            // Phase 2 buffered-release cleanup must use the key identity alone.
            let transition_p = is_transition_p(&event);
            let phase_transition_key = phase.starts_with("Phase 1") && transition_p;
            let phase_transition_release = phase_transition_key && is_transition_p_release(&event);
            let exit = matches!(
                &event,
                Event::Key(key) if key.code == KeyCode::Esc && key.kind == KeyEventKind::Press
            );
            if ignore_next_transition_release {
                ignore_next_transition_release = false;
                if is_transition_p_release(&event) {
                    println!("Ignoring the buffered Phase 1 transition P release in Phase 2");
                    continue;
                }
            }
            if exit {
                stats.clear_duplicate_state();
                return Ok((false, false));
            }
            let transition_press = transition_on_press
                && phase_transition_key
                && matches!(&event, Event::Key(key) if key.kind == KeyEventKind::Press);
            stats.record(&event);
            if transition_press {
                // This control Press may have no observable Release on a VT transport; keep it
                // in raw counts but exclude it from the balance diagnostic.
                stats.cancel_control_press(&event);
            }
            if phase_transition_pending {
                if phase_transition_release {
                    return Ok((true, false));
                }
            } else if phase_transition_key
                && matches!(&event, Event::Key(key) if key.kind == KeyEventKind::Press)
            {
                if transition_on_press {
                    return Ok((true, true));
                }
                phase_transition_pending = true;
            }
        }
    }

    fn run_session(initial_poll_result: bool) -> io::Result<()> {
        let mut stdout = io::stdout();
        let mut raw_enabled = false;
        let mut mouse_cleanup_required = false;
        let mut paste_cleanup_required = false;
        let mut phase_one_stats = SessionStats::default();
        let mut phase_two_stats = SessionStats::default();

        let session_result = (|| {
            drain_startup_events(initial_poll_result)?;
            let mode_before_raw = current_console_mode()?;
            enable_raw_mode()?;
            raw_enabled = true;

            let raw_mode = current_console_mode()?;
            print_mode("after enable_raw_mode", raw_mode);
            let vt_preserved = (mode_before_raw & ENABLE_VIRTUAL_TERMINAL_INPUT)
                == (raw_mode & ENABLE_VIRTUAL_TERMINAL_INPUT);
            println!(
                "raw-mode VT_INPUT preservation: {}",
                if vt_preserved { "PASS" } else { "FAIL" }
            );
            if !vt_preserved {
                return Err(io::Error::other(
                    "raw mode changed ENABLE_VIRTUAL_TERMINAL_INPUT",
                ));
            }
            let phase_one_label = if mode_before_raw & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
                "Phase 1: ordinary Win32 input (paste disabled)"
            } else {
                "Phase 1: original input transport (paste disabled)"
            };
            if mode_before_raw & ENABLE_VIRTUAL_TERMINAL_INPUT != 0 {
                println!(
                    "Phase 1 preserves pre-existing VT input; Win32 Release semantics cannot be isolated in this session"
                );
            }
            println!(
                "mouse mode bits after raw mode: MOUSE_INPUT={}, WINDOW_INPUT={}",
                raw_mode & ENABLE_MOUSE_INPUT != 0,
                raw_mode & ENABLE_WINDOW_INPUT != 0
            );
            mouse_cleanup_required = true;
            execute!(stdout, EnableMouseCapture)?;
            let mouse_mode = current_console_mode()?;
            print_mode("after EnableMouseCapture", mouse_mode);
            if mode_before_raw & ENABLE_VIRTUAL_TERMINAL_INPUT == 0 {
                println!(
                    "Phase 1 (paste disabled): press ordinary keys, F1, Ctrl+Shift+B, and use Win32 mouse. Press and release P to continue, or press Esc to exit."
                );
            } else {
                println!(
                    "Phase 1 (paste disabled): exercise the original input transport with ordinary keys, F1, Ctrl+Shift+B, and mouse. Press and release P to continue, or press Esc to exit."
                );
            }
            let (continue_to_paste, ignore_transition_release) = read_phase(
                &mut phase_one_stats,
                phase_one_label,
                mode_before_raw & ENABLE_VIRTUAL_TERMINAL_INPUT != 0,
                false,
            )?;
            phase_one_stats.print_summary();
            if !continue_to_paste {
                return Ok(());
            }

            paste_cleanup_required = true;
            execute!(stdout, EnableBracketedPaste)?;
            let paste_mode = current_console_mode()?;
            print_mode("after EnableBracketedPaste", paste_mode);
            println!(
                "Phase 2 (paste enabled): paste multiline text, use SGR mouse, and type physical numeric-keypad Alt+0233. Windows Terminal/ConPTY may report key Press without Release while VT input is active. Press Esc to exit."
            );
            let _ = read_phase(
                &mut phase_two_stats,
                "Phase 2: VT transport (bracketed paste enabled)",
                false,
                ignore_transition_release,
            )?;
            phase_two_stats.print_summary();

            Ok(())
        })();

        // Cleanup is deliberately unconditional: a command or read can fail after raw mode or
        // bracketed paste has already been enabled.
        let mut cleanup_error = None;
        if mouse_cleanup_required {
            if let Err(error) = execute!(stdout, DisableMouseCapture) {
                cleanup_error = Some(error);
            }
        }
        if paste_cleanup_required {
            if let Err(error) = execute!(stdout, DisableBracketedPaste) {
                cleanup_error.get_or_insert(error);
            }
        }
        let disable_raw_result = if raw_enabled {
            disable_raw_mode()
        } else {
            Ok(())
        };
        if let Err(error) = disable_raw_result {
            cleanup_error.get_or_insert(error);
        }
        if let Err(error) = session_result {
            return Err(error);
        }
        if let Some(error) = cleanup_error {
            return Err(error);
        }
        Ok(())
    }

    pub(super) fn run() -> io::Result<()> {
        println!("Windows VT input diagnostic");
        let original_mode = current_console_mode()?;
        print_mode("before event::poll(Duration::ZERO)", original_mode);

        // This constructs the shared EVENT_READER and its Windows event source before raw mode.
        let poll_result = poll(Duration::ZERO)?;
        let after_poll_mode = current_console_mode()?;
        print_mode("after event::poll(Duration::ZERO)", after_poll_mode);
        println!(
            "event source construction mode preservation: {}",
            if original_mode == after_poll_mode {
                "PASS"
            } else {
                "FAIL"
            }
        );
        println!("initial poll result: {poll_result}");

        let session_result = run_session(poll_result);

        let final_mode_result = current_console_mode();
        match final_mode_result {
            Ok(final_mode) => {
                print_mode("after cleanup", final_mode);
                let restore_mask = ENABLE_VIRTUAL_TERMINAL_INPUT
                    | ENABLE_MOUSE_INPUT
                    | ENABLE_WINDOW_INPUT
                    | ENABLE_EXTENDED_FLAGS;
                let restored = final_mode & restore_mask == original_mode & restore_mask;
                println!(
                    "original VT_INPUT/MOUSE_INPUT/WINDOW_INPUT/EXTENDED_FLAGS restoration: {}",
                    if restored { "PASS" } else { "FAIL" }
                );
                if !restored && session_result.is_ok() {
                    return Err(io::Error::other(
                        "console input mode bits were not restored after diagnostic cleanup",
                    ));
                }
            }
            Err(error) => {
                println!("after cleanup: unable to read console mode: {error}");
                println!("original console input mode restoration: FAIL");
                if session_result.is_ok() {
                    return Err(error);
                }
            }
        }

        session_result
    }
}

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    windows::run()
}

#[cfg(not(windows))]
fn main() {
    println!("windows-vt-input is only available on Windows.");
}
