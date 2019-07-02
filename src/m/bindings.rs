use buffer::Mark;
use command::{Action, Command, Operation};
use key::Key;
use textobject::{Anchor, Kind, Offset, TextObject};

pub fn handle_key_event(key: Key) -> Command {
    match key {
        // Editor Commands
        Key::Ctrl('q') => Command::exit_editor(),
        Key::Ctrl('s') => Command::save_buffer(),
        Key::Ctrl('o') => Command::force_exit_editor(),

        Key::Ctrl('d') => Command::duplicate_selection(),
        Key::Ctrl('k') => Command::delete_selection(),
        Key::Ctrl('x') => Command::cut_selection(),
        Key::Ctrl('c') => Command::copy_selection(),
        Key::Ctrl('v') => Command::paste(),
        Key::CtrlUp => Command::move_selection(false),
        Key::CtrlDown => Command::move_selection(true),

        Key::Ctrl('z') => Command::undo(),
        Key::Ctrl('y') => Command::redo(),


        Key::Char(c) => Command::insert_char(c),

        // Cursor movement
        Key::Up => Command::movement(
            Offset::Backward(1, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),

        Key::Down => Command::movement(
            Offset::Forward(1, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),

        Key::PgUp => Command::movement(
            Offset::Backward(10, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),

        Key::PgDown => Command::movement(
            Offset::Forward(10, Mark::Cursor(0)),
            Kind::Line(Anchor::Same),
        ),

        Key::Left => Command::movement(Offset::Backward(1, Mark::Cursor(0)), Kind::Char),
        Key::Right => Command::movement(Offset::Forward(1, Mark::Cursor(0)), Kind::Char),

        Key::CtrlRight => Command::movement(
            Offset::Forward(1, Mark::Cursor(0)),
            Kind::Word(Anchor::Start),
        ),
        Key::CtrlLeft => Command::movement(
            Offset::Backward(1, Mark::Cursor(0)),
            Kind::Word(Anchor::Start),
        ),

        Key::End => Command::movement(Offset::Forward(0, Mark::Cursor(0)), Kind::Line(Anchor::End)),
        Key::Home => Command::movement(
            Offset::Backward(0, Mark::Cursor(0)),
            Kind::Line(Anchor::Start),
        ),

        // Editing
        Key::Tab => Command::insert_tab(),
        Key::Enter => Command::insert_char('\n'),
        Key::Backspace => Command {
            number: 1,
            action: Action::Operation(Operation::DeleteFromMark(Mark::Cursor(0))),
            object: Some(TextObject {
                kind: Kind::Char,
                offset: Offset::Backward(1, Mark::Cursor(0)),
            }),
        },
        Key::Delete => Command {
            number: 1,
            action: Action::Operation(Operation::DeleteFromMark(Mark::Cursor(0))),
            object: Some(TextObject {
                kind: Kind::Char,
                offset: Offset::Forward(1, Mark::Cursor(0)),
            }),
        },


        // History
        _ => Command::noop(),
    }
}
