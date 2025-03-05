use super::Project;
use crate::screen::ScreenInstance;

/// A game that is currently running.
///
/// A [`Project`] is like a recipe for a [`Game`]:
/// A Game instantiates all the markers and descriptors of a project
/// into an interactive form.
pub struct Game<'game> {
    pub project: &'game Project,
    pub current_scene: Option<ScreenInstance<'game>>,
}

impl<'game> Game<'game> {
    /// Reifies a game from a project. This only _creates_
    /// an instace of a game with appropriate handles to resources, etc,
    /// it doesn't make the game start playing.
    pub fn from_project(project: &Project) -> Game {
        Game {
            project,
            current_scene: None,
        }
    }

    /// Calls the project's startup behaviour to set up and finally begin to play the game.
    pub fn game_started(&mut self) {
        self.project.startup_behaviour.reify().execute();
    }
}
