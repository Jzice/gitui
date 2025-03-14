mod changes;
mod command;
mod commit;
mod commit_details;
mod commitlist;
mod create_branch;
mod diff;
mod externaleditor;
mod filetree;
mod help;
mod inspect_commit;
mod msg;
mod push;
mod reset;
mod stashmsg;
mod tag_commit;
mod textinput;
mod utils;

use anyhow::Result;
use crossterm::event::Event;

pub use changes::ChangesComponent;
pub use command::{CommandInfo, CommandText};
pub use commit::CommitComponent;
pub use commit_details::CommitDetailsComponent;
pub use commitlist::CommitList;
pub use create_branch::CreateBranchComponent;
pub use diff::DiffComponent;
pub use externaleditor::ExternalEditorComponent;
pub use filetree::FileTreeComponent;
pub use help::HelpComponent;
pub use inspect_commit::InspectCommitComponent;
pub use msg::MsgComponent;
pub use push::PushComponent;
pub use reset::ResetComponent;
pub use stashmsg::StashMsgComponent;
pub use tag_commit::TagCommitComponent;
pub use textinput::TextInputComponent;
pub use utils::filetree::FileTreeItemKind;

use crate::ui::style::Theme;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Text},
    Frame,
};

/// creates accessors for a list of components
///
/// allows generating code to make sure
/// we always enumerate all components in both getter functions
#[macro_export]
macro_rules! accessors {
    ($self:ident, [$($element:ident),+]) => {
        fn components(& $self) -> Vec<&dyn Component> {
            vec![
                $(&$self.$element,)+
            ]
        }

        fn components_mut(&mut $self) -> Vec<&mut dyn Component> {
            vec![
                $(&mut $self.$element,)+
            ]
        }
    };
}

/// returns `true` if event was consumed
pub fn event_pump(
    ev: Event,
    components: &mut [&mut dyn Component],
) -> Result<bool> {
    for c in components {
        if c.event(ev)? {
            return Ok(true);
        }
    }

    Ok(false)
}

/// helper fn to simplify delegating command
/// gathering down into child components
/// see `event_pump`,`accessors`
pub fn command_pump(
    out: &mut Vec<CommandInfo>,
    force_all: bool,
    components: &[&dyn Component],
) {
    for c in components {
        if c.commands(out, force_all) != CommandBlocking::PassingOn
            && !force_all
        {
            break;
        }
    }
}

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}

///
#[derive(PartialEq)]
pub enum CommandBlocking {
    Blocking,
    PassingOn,
}

///
pub fn visibility_blocking<T: Component>(
    comp: &T,
) -> CommandBlocking {
    if comp.is_visible() {
        CommandBlocking::Blocking
    } else {
        CommandBlocking::PassingOn
    }
}

///
pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()>;
}

/// base component trait
pub trait Component {
    ///
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking;

    /// returns true if event propagation needs to end (event was consumed)
    fn event(&mut self, ev: Event) -> Result<bool>;

    ///
    fn focused(&self) -> bool {
        false
    }
    /// focus/unfocus this component depending on param
    fn focus(&mut self, _focus: bool) {}
    ///
    fn is_visible(&self) -> bool {
        true
    }
    ///
    fn hide(&mut self) {}
    ///
    fn show(&mut self) -> Result<()> {
        Ok(())
    }

    ///
    fn toggle_visible(&mut self) -> Result<()> {
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.show()
        }
    }
}

fn dialog_paragraph<'a, 't, T>(
    title: &'a str,
    content: T,
    theme: &Theme,
    focused: bool,
) -> Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    Paragraph::new(content)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .title_style(theme.title(focused))
                .border_style(theme.block(focused)),
        )
        .alignment(Alignment::Left)
}

fn popup_paragraph<'a, 't, T>(
    title: &'a str,
    content: T,
    theme: &Theme,
    focused: bool,
) -> Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    Paragraph::new(content)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title_style(theme.title(focused))
                .border_style(theme.block(focused)),
        )
        .alignment(Alignment::Left)
        .wrap(true)
}
