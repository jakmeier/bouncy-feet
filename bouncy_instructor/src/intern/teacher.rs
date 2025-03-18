use super::step_pace::StepPace;
use crate::skeleton::Cartesian2d;
use crate::tracker::{DanceCursor, TeacherView};
use crate::StepInfo;

/// Dynamically switch between steps, switch between views for showing the next
/// step and going back to full camera mode for dancers to see themselves.
#[derive(Default)]
pub(crate) struct Teacher {
    sections: Vec<Section>,
    // TODO: audio hints by the teacher
}

enum Section {
    /// A step to dance by the student.
    Step(StepSection),
    /// The instructor shows the step, the student can watch.
    ShowStep(StepSection),
    /// Any move allowed from the student.
    Freestyle { subbeats: u32 },
    /// The instructor shows some move but the student doesn't have to follow
    /// exactly. Just make sure to always move.
    Warmup(StepSection),
}

struct StepSection {
    step: StepInfo,
    subbeats: u32,
    pace: StepPace,
}

impl Teacher {
    /// A step to dance by the student.
    pub(crate) fn add_step(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Step(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
    }

    /// The instructor shows the step, the student can watch.
    pub(crate) fn show_step(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::ShowStep(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
    }

    /// Any move allowed from the student.
    pub(crate) fn add_freestyle(&mut self, beats: u32) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Freestyle {
            subbeats: beats * 2,
        });
    }

    /// Show a step to copy by the student but be more lenient in tracking.
    pub(crate) fn add_warmup(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Warmup(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
    }

    pub(crate) fn step(&self, cursor: &DanceCursor) -> Option<&StepInfo> {
        self.section(cursor).and_then(|section| match &section {
            Section::Step(step_section)
            | Section::ShowStep(step_section)
            | Section::Warmup(step_section) => {
                let StepSection { step, .. } = step_section;
                Some(step)
            }
            Section::Freestyle { .. } => None,
        })
    }

    /// After n subbeats of the lesson, get the current step and beat remainder.
    ///
    /// Full beat: only count 1,2,3,4 (used for bpm calculation)
    /// Sub beat: also count the "and" between
    pub(crate) fn step_at_subbeat(&self, subbeat: u32) -> Option<(&StepInfo, u32)> {
        self.section_at_subbeat(subbeat)
            .and_then(|section| match &section {
                Section::Step(step_section)
                | Section::ShowStep(step_section)
                | Section::Warmup(step_section) => {
                    let StepSection { step, pace, .. } = step_section;
                    Some((step, pace.pose_at_subbeat(subbeat)))
                }
                Section::Freestyle { .. } => None,
            })
    }

    pub(crate) fn cursor_at_subbeat(&self, subbeat: u32) -> DanceCursor {
        let (step_index, remainder) = self.index_at_subbeat(subbeat);
        let section = self.sections.get(step_index);
        let pose_index = section
            .map(|section| match section {
                Section::Step(step_section)
                | Section::ShowStep(step_section)
                | Section::Warmup(step_section) => {
                    let StepSection { pace, .. } = step_section;
                    pace.pose_at_subbeat(remainder as u32) as usize
                }
                Section::Freestyle { .. } => 0,
            })
            .unwrap_or(0);
        DanceCursor {
            subbeat,
            step_index,
            pose_index,
        }
    }

    fn index_at_subbeat(&self, subbeat: u32) -> (usize, usize) {
        let mut subbeat_remainder = subbeat;
        for (index, section) in self.sections.iter().enumerate() {
            if subbeat_remainder < section.subbeats() {
                return (index, subbeat_remainder as usize);
            }
            subbeat_remainder -= section.subbeats();
        }
        (self.sections.len(), 0)
    }

    fn section(&self, cursor: &DanceCursor) -> Option<&Section> {
        self.sections.get(cursor.step_index)
    }

    fn section_at_subbeat(&self, mut subbeat: u32) -> Option<&Section> {
        for section in &self.sections {
            if subbeat < section.subbeats() {
                return Some(section);
            }
            subbeat -= section.subbeats();
        }
        None
    }

    /// How far the body position has moved from the origin at a specific beat.
    pub(crate) fn pose_body_shift_at_subbeat(&self, mut subbeat: u32) -> Cartesian2d {
        let mut shift = Cartesian2d::default();
        for section in &self.sections {
            let beats_on_step = u32::min(section.subbeats(), subbeat);
            shift = shift + section.body_shift(beats_on_step);
            if subbeat < section.subbeats() {
                break;
            }
            subbeat -= section.subbeats();
        }
        shift
    }

    /// Whether at the given subbeat, the student should be tracked.
    pub(crate) fn is_tracking_at_subbeat(&self, subbeat: u32) -> bool {
        self.section_at_subbeat(subbeat)
            .map(|section| match &section {
                Section::Step(_) | Section::Freestyle { .. } | Section::Warmup(_) => true,
                Section::ShowStep(_) => false,
            })
            .unwrap_or_default()
    }

    /// What information the UI should show.
    pub(crate) fn ui_view_at_subbeat(&self, subbeat: u32) -> TeacherView {
        self.section_at_subbeat(subbeat)
            .map(|section| match &section {
                Section::Step(_) => TeacherView::InstructorAndCamera,
                Section::Freestyle { .. } => TeacherView::UserCameraWithTracking,
                Section::Warmup(_) | Section::ShowStep(_) => TeacherView::InstructorOnly,
            })
            .unwrap_or(TeacherView::Off)
    }

    /// The teacher is done when there is no more to show or track.
    pub(crate) fn is_done(&self, subbeat: u32) -> bool {
        self.section_at_subbeat(subbeat).is_none()
    }

    /// How many beats to track for
    pub(crate) fn tracked_subbeats(&self) -> u32 {
        self.sections.iter().map(|section| section.subbeats()).sum()
    }
}

impl Section {
    fn subbeats(&self) -> u32 {
        match self {
            Section::Step(step_section)
            | Section::ShowStep(step_section)
            | Section::Warmup(step_section) => {
                let StepSection { subbeats, .. } = step_section;
                *subbeats
            }
            Section::Freestyle { subbeats } => *subbeats,
        }
    }

    fn body_shift(&self, beat: u32) -> Cartesian2d {
        match self {
            Section::Step(step_section)
            | Section::ShowStep(step_section)
            | Section::Warmup(step_section) => {
                let StepSection { step, .. } = step_section;
                step.body_shift(beat as usize)
            }
            Section::Freestyle { .. } => Cartesian2d::default(),
        }
    }
}
