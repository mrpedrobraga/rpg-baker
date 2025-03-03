use super::Project;

/// A game that is currently running.
///
/// A [`Project`] is like a recipe for a [`Game`]:
/// A Game instantiates all the markers and descriptors of a project
/// into an interactive form.
pub struct Game<'runtime> {
    project: &'runtime Project,
}

impl<'runtime> Game<'runtime> {
    pub fn from_project(project: &Project) -> Game {
        Game { project }
    }

    pub fn do_runtime_routine(&mut self) {
        let routine = self.project.startup_routine.blocks.blocks.lock_ref();

        // TODO: Replace this with a dedicated function somewhere else
        // for running routines in different contexts (with proper flow control).
        for entry in routine.iter() {
            let block = entry.reify().expect("Failure reifying!");
            let _result = block.evaluate();
        }
    }
}
