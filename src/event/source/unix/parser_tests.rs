use crate::event::internal::InternalEvent;
use crate::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::{Parser, TTY_BUFFER_SIZE};

fn key(code: KeyCode, modifiers: KeyModifiers) -> InternalEvent {
    InternalEvent::Event(Event::Key(KeyEvent::new(code, modifiers)))
}

fn read_events(chunks: &[&[u8]]) -> Vec<InternalEvent> {
    let mut parser = Parser::default();
    for chunk in chunks {
        parser.advance(chunk, chunk.len() == TTY_BUFFER_SIZE);
    }
    assert!(parser.buffer.is_empty(), "{:?}", parser.buffer);
    parser.collect()
}

#[test]
fn escape_prefixed_csi_and_ss3_keys() {
    let cases: &[(&[u8], KeyCode, KeyModifiers)] = &[
        (b"\x1B\x1B[A", KeyCode::Up, KeyModifiers::ALT),
        (b"\x1B\x1B[B", KeyCode::Down, KeyModifiers::ALT),
        (b"\x1B\x1B[C", KeyCode::Right, KeyModifiers::ALT),
        (b"\x1B\x1B[D", KeyCode::Left, KeyModifiers::ALT),
        (b"\x1B\x1BOA", KeyCode::Up, KeyModifiers::ALT),
        (b"\x1B\x1BOH", KeyCode::Home, KeyModifiers::ALT),
        (b"\x1B\x1BOP", KeyCode::F(1), KeyModifiers::ALT),
        (
            b"\x1B\x1B[1;2A",
            KeyCode::Up,
            KeyModifiers::ALT | KeyModifiers::SHIFT,
        ),
    ];
    for &(bytes, code, modifiers) in cases {
        assert_eq!(
            read_events(&[bytes]),
            vec![key(code, modifiers)],
            "{bytes:?}"
        );
    }
}

#[test]
fn repeated_escapes_keep_existing_events() {
    for count in [1, 2, 3, 4, 5, 16, TTY_BUFFER_SIZE] {
        let bytes = vec![b'\x1B'; count];
        // Preserve the existing behavior: each adjacent pair produces one Esc.
        assert_eq!(
            read_events(&[&bytes]),
            vec![key(KeyCode::Esc, KeyModifiers::NONE); count.div_ceil(2)],
            "{count} escapes",
        );
    }
}

#[test]
fn double_escape_followed_by_an_ordinary_key() {
    let cases: &[(&[u8], KeyCode, KeyModifiers)] = &[
        (b"\x1B\x1Ba", KeyCode::Char('a'), KeyModifiers::NONE),
        (b"\x1B\x1BA", KeyCode::Char('A'), KeyModifiers::SHIFT),
        (b"\x1B\x1B\r", KeyCode::Enter, KeyModifiers::NONE),
        (b"\x1B\x1B\t", KeyCode::Tab, KeyModifiers::NONE),
        (b"\x1B\x1B\x14", KeyCode::Char('t'), KeyModifiers::CONTROL),
        (
            "\x1B\x1Bé".as_bytes(),
            KeyCode::Char('é'),
            KeyModifiers::NONE,
        ),
    ];
    for &(bytes, code, modifiers) in cases {
        assert_eq!(
            read_events(&[bytes]),
            vec![key(KeyCode::Esc, KeyModifiers::NONE), key(code, modifiers)],
            "{bytes:?}",
        );
    }
}

#[test]
fn neighboring_events_keep_their_order() {
    assert_eq!(
        read_events(&[b"a\x1B\x1B[A\x1B\x1BOBb"]),
        vec![
            key(KeyCode::Char('a'), KeyModifiers::NONE),
            key(KeyCode::Up, KeyModifiers::ALT),
            key(KeyCode::Down, KeyModifiers::ALT),
            key(KeyCode::Char('b'), KeyModifiers::NONE),
        ],
    );
}

#[test]
fn sequences_continue_after_the_introducer() {
    let cases: &[(&[u8], &[u8])] = &[(b"\x1B\x1B[", b"A"), (b"\x1B\x1BO", b"A")];
    for &(prefix, suffix) in cases {
        let mut parser = Parser::default();
        parser.advance(prefix, false);
        assert_eq!(parser.next(), None);
        parser.advance(suffix, false);
        assert_eq!(
            parser.collect::<Vec<_>>(),
            vec![key(KeyCode::Up, KeyModifiers::ALT)],
        );
    }
}

#[test]
fn early_read_boundaries_keep_existing_events() {
    assert_eq!(
        read_events(&[b"\x1B", b"\x1B[A"]),
        vec![
            key(KeyCode::Esc, KeyModifiers::NONE),
            key(KeyCode::Up, KeyModifiers::NONE)
        ],
    );
    assert_eq!(
        read_events(&[b"\x1B\x1B", b"[A"]),
        vec![
            key(KeyCode::Esc, KeyModifiers::NONE),
            key(KeyCode::Char('['), KeyModifiers::NONE),
            key(KeyCode::Char('A'), KeyModifiers::SHIFT),
        ],
    );
    assert_eq!(
        read_events(&[b"\x1B", b"\x1B", b"[", b"A"]),
        vec![
            key(KeyCode::Esc, KeyModifiers::NONE),
            key(KeyCode::Esc, KeyModifiers::NONE),
            key(KeyCode::Char('['), KeyModifiers::NONE),
            key(KeyCode::Char('A'), KeyModifiers::SHIFT),
        ],
    );
}

#[test]
fn full_read_with_a_trailing_escape() {
    let mut first = vec![b'x'; TTY_BUFFER_SIZE - 1];
    first.push(b'\x1B');
    let mut expected = vec![key(KeyCode::Char('x'), KeyModifiers::NONE); TTY_BUFFER_SIZE - 1];
    expected.push(key(KeyCode::Up, KeyModifiers::ALT));
    assert_eq!(read_events(&[&first, b"\x1B[A"]), expected);
}

#[test]
fn full_read_with_two_trailing_escapes() {
    let mut first = vec![b'x'; TTY_BUFFER_SIZE - 2];
    first.extend_from_slice(b"\x1B\x1B");
    let mut expected = vec![key(KeyCode::Char('x'), KeyModifiers::NONE); TTY_BUFFER_SIZE - 2];
    expected.extend([
        key(KeyCode::Esc, KeyModifiers::NONE),
        key(KeyCode::Char('['), KeyModifiers::NONE),
        key(KeyCode::Char('A'), KeyModifiers::SHIFT),
    ]);
    assert_eq!(read_events(&[&first, b"[A"]), expected);
}

#[test]
fn standard_alt_keys() {
    assert_eq!(
        read_events(&[b"\x1B[1;3A\x1Ba"]),
        vec![
            key(KeyCode::Up, KeyModifiers::ALT),
            key(KeyCode::Char('a'), KeyModifiers::ALT)
        ],
    );
}
