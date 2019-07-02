use buffer::Mark;
use textobject::{Kind, Offset, TextObject};
use command::Operation::Insert;


/// Instructions for the Editor.
/// These do NOT alter the text, but may change editor/view state
#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    SaveBuffer,
    //FindFile,
    ExitEditor,
    // force exit
    ForceExitEditor,

    SetMark(Mark),
    ShowMessage(&'static str),
    None,
}

/// Operations on the Buffer.
/// These DO alter the text, but otherwise may NOT change editor/view state
/// Note that these differ from `log::Change` in that they are higher-level
/// operations dependent on state (cursor/mark locations, etc.), as opposed
/// to concrete operations on absolute indexes (insert 'a' at index 158, etc.)
#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Insert(char),         // insert text
    DeleteFromMark(Mark), // delete from some mark to an object
    DuplicateSelection,   // duplicate the selection
    DeleteSelection,      // delete the selection
    CutSelection,         // cut the selection from the buffer to the clipboard
    CopySelection,        // copy the selection to the clipboard
    Paste,                // insert the clipboard
    MoveSelection(bool),  //Move the current selection up or down

    Undo, // rewind buffer transaction log
    Redo, // replay buffer transaction log
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Operation(Operation),
    Instruction(Instruction),
}

/// A complete, actionable command
#[derive(Copy, Clone, Debug)]
pub struct Command {
    pub number: i32,                // numeric paramter, line number, repeat count, etc.
    pub action: Action,             // what to do
    pub object: Option<TextObject>, // where to do it
}

impl Command {
    /// Display a message
    pub fn show_message(msg: &'static str) -> Command {
        Command {
            action: Action::Instruction(Instruction::ShowMessage(msg)),
            number: 0,
            object: None,
        }
    }

    /// Shortcut to create an ExitEditor command
    pub fn exit_editor() -> Command {
        Command {
            action: Action::Instruction(Instruction::ExitEditor),
            number: 0,
            object: None,
        }
    }

    pub fn force_exit_editor() -> Command{
        Command{
            action: Action::Instruction(Instruction::ForceExitEditor),
            number: 0,
            object: None,
        }
    }

    /// Shortcut to create a SaveBuffer command
    /// number indicates what? it's not 0 here, 0 IS INSTRUCTION AND 1 IS OPERATION
    pub fn save_buffer() -> Command {
        // save buffer why exit?
        Command {
            action: Action::Instruction(Instruction::SaveBuffer),
            number: 0,
            object: None,
        }
    }

    /// Shortcut to create an Insert command
    pub fn insert_char(c: char) -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::Insert(c)),
            object: None,
        }
    }

    /// Shortcut to create an Insert command
    // FIXME: shouldn't need this method
    pub fn insert_tab() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::Insert('\t')),
            object: None,
        }
    }

    /// Shortcut to create DeleteSelection command
    pub fn delete_selection() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::DeleteSelection),
            object: None,
        }
    }

    /// Shortcut to create DuplicateSelection command
    pub fn duplicate_selection() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::DuplicateSelection),
            object: None,
        }
    }

    /// Shortcut to create CutSelection command
    pub fn cut_selection() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::CutSelection),
            object: None,
        }
    }

    /// Shortcut to create Paste command
    pub fn paste() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::Paste),
            object: None,
        }
    }

    /// Shortcut to create CopySelection command
    pub fn copy_selection() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::CopySelection),
            object: None,
        }
    }

    pub fn move_selection(down: bool) -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::MoveSelection(down)),
            object: None,
        }
    }

    /// Shortcut to create Undo command
    pub fn undo() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::Undo),
            object: None,
        }
    }

    /// Shortcut to create Redo command
    pub fn redo() -> Command {
        Command {
            number: 1,
            action: Action::Operation(Operation::Redo),
            object: None,
        }
    }

    pub fn movement(offset: Offset, kind: Kind) -> Command {
        Command {
            number: 1,
            action: Action::Instruction(Instruction::SetMark(Mark::Cursor(0))),
            object: Some(TextObject {
                kind: kind,
                offset: offset,
            }),
        }
    }

    pub fn noop() -> Command {
        Command {
            number: 0,
            action: Action::Instruction(Instruction::None),
            object: None,
        }
    }
}
