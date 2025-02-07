use crate::skeleton::Cartesian2d;
use crate::StepInfo;

use super::step_pace::StepPace;

/// Dynamically switch between steps, switch between views for showing the next
/// step and going back to full camera mode for dancers to see themselves.
#[derive(Default)]
pub(crate) struct Teacher {
    sections: Vec<Section>,
}

enum Section {
    Step {
        step: StepInfo,
        subbeats: u32,
        pace: StepPace,
    },
    Freestyle {
        subbeats: u32,
    },
}

impl Teacher {
    pub(crate) fn add_step(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Step {
            step,
            subbeats: beats * 2,
            pace,
        });
    }

    pub(crate) fn add_freestyle(&mut self, beats: u32) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Freestyle {
            subbeats: beats * 2,
        });
    }

    /// After n subbeats of the lesson, get the current step and beat remainder.
    ///
    /// Full beat: only count 1,2,3,4 (used for bpm calculation)
    /// Sub beat: also count the "and" between
    pub(crate) fn step_at_subbeat(&self, mut subbeat: u32) -> Option<(&StepInfo, u32)> {
        for section in &self.sections {
            if subbeat < section.subbeats() {
                match &section {
                    Section::Step { step, pace, .. } => {
                        return Some((step, pace.pose_at_subbeat(subbeat)));
                    }
                    Section::Freestyle { .. } => {
                        return None;
                    }
                }
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

    /// How many beats to track for
    pub(crate) fn tracked_subbeats(&self) -> u32 {
        self.sections.iter().map(|section| section.subbeats()).sum()
    }
}

impl Section {
    fn subbeats(&self) -> u32 {
        match self {
            Section::Step { subbeats, .. } => *subbeats,
            Section::Freestyle { subbeats } => *subbeats,
        }
    }

    fn body_shift(&self, beat: u32) -> Cartesian2d {
        match self {
            Section::Step { step, .. } => step.body_shift(beat as usize),
            Section::Freestyle { .. } => Cartesian2d::default(),
        }
    }
}
