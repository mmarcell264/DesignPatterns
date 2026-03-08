use cursive::{
    traits::Nameable,
    views::{Dialog, EditView},
    Cursive
};

use super::{command::Command, copy::CopyCommand, cut::CutCommand, paste::PasteCommand};

/// An application context to be passed into visual component callbacks.
/// It contains a clipboard and a history of commands to be undone.
#[derive(Default)]
pub struct AppContext {
    pub clipboard: String,
    history: Vec<Box<dyn Command>>
}

/// Executes a command and then pushes it to a history array.
fn execute(app: & mut Cursive, mut command: impl Command + 'static){
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command));
        });
    }
}

/// Pops the last command and executes an undo action.
fn undo(app: & mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app);
    }
    app.set_user_data(context);
}

pub fn command_main() {
    let mut app = cursive::default();
    
    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and usse buttons")
            .button("Copy", |s| execute(s, CopyCommand))
            .button("Cut", |s| execute(s, CutCommand::default()))
            .button("Paste", |s| execute(s, PasteCommand::default()))
            .button("Undo", undo)
            .button("Quit", |s| s.quit()),

    );

    app.run();
}