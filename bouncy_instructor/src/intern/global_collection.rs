use std::rc::Rc;

use crate::wrapper::pose_wrapper::PoseWrapper;
use crate::wrapper::step_wrapper::StepWrapper;
use crate::{dance_file, step_file, AddDanceError, AddStepError};

use super::step::StepSource;
use super::tracker_dance_collection::{AddPoseError, TrackerDanceCollection};

/// List of all registered data, in their source-of-truth form with handles for
/// cheap WASM-to-JS transitions.
///
/// There is a singleton GlobalCollection per WASM module instance (or process,
/// if running outside the web). Loading poses, steps, dances and courses into
/// the instructor always happens through this global data store.
///
/// Accessing data from this collection is good for anything that is not super
/// performance critical. For tracking, there is the more optimized
/// [`TrackerDanceCollection`](crate::intern::tracker_dance_collection::TrackerDanceCollection)
/// view, which usually contains a subset of what is in the global instance.
///
/// On translations, this structure keeps strings for all available languages.
#[derive(Debug, Default)]
pub(crate) struct GlobalCollection {
    poses: Vec<PoseWrapper>,
    steps: Vec<StepWrapper>,
    // TODO: should use wrapper
    dances: Vec<dance_file::Dance>,
    // TODO: should use wrapper
    // courses: Vec<course_file::CourseFile>,
    //
    /// A convenient tracker dance collection that includes everything from the
    /// global collection.
    ///
    /// Keeping this around is useful to create trackers and sharing a
    /// reference-counted collection among them. Plus, it makes it easy to
    /// validate inputs when they are added to the global collection. When this
    /// global instance is mutated, clone-on-write is used using `Rc::make_mut`.
    ///
    /// TODO: this is used more than it should be, refactor to remove direct
    /// references, then make this private
    pub(crate) tracker_view: Rc<TrackerDanceCollection>,
}

impl GlobalCollection {
    pub(crate) fn add_poses(
        &mut self,
        poses: Vec<crate::pose_file::Pose>,
    ) -> Result<(), AddPoseError> {
        Rc::make_mut(&mut self.tracker_view).add_poses(poses.iter())?;
        self.poses.extend(poses.into_iter().map(PoseWrapper::new));
        Ok(())
    }

    pub(crate) fn add_steps(
        &mut self,
        steps: impl IntoIterator<Item = step_file::Step> + AsRef<[step_file::Step]>,
        source: StepSource,
    ) -> Result<(), AddStepError> {
        Rc::make_mut(&mut self.tracker_view).add_steps(steps.as_ref().iter(), source.clone())?;
        self.steps.extend(
            steps
                .into_iter()
                .map(|def| StepWrapper::new(def, source.clone())),
        );
        Ok(())
    }

    pub(crate) fn add_dances(
        &mut self,
        mut dances: Vec<dance_file::Dance>,
    ) -> Result<(), AddDanceError> {
        Rc::make_mut(&mut self.tracker_view).add_dances(dances.iter())?;
        self.dances.append(&mut dances);
        Ok(())
    }

    /// Change language and reload the tracker view on it.
    pub(crate) fn set_language(&mut self, lang: String) {
        let mut db = TrackerDanceCollection::new(lang);
        const EXPECT: &str = "input checked previously";

        db.add_poses(self.poses.iter().map(|p| p.definition()))
            .expect(EXPECT);
        for step in &self.steps {
            db.add_steps(std::iter::once(step.definition()), step.source().clone())
                .expect(EXPECT);
        }
        db.add_dances(self.dances.iter()).expect(EXPECT);

        self.tracker_view = Rc::new(db);
    }
}
