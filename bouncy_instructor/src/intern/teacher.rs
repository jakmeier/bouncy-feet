use crate::skeleton::Cartesian2d;
use crate::StepInfo;

/// Dynamically switch between steps, switch between views for showing the next
/// step and going back to full camera mode for dancers to see themselves.
#[derive(Default)]
pub(crate) struct Teacher {
    sections: Vec<Section>,
}

enum Section {
    Step { step: StepInfo, beats: u32 },
    Freestyle { beats: u32 },
    // more for later:
    //     speed per section
}

impl Teacher {
    pub(crate) fn add_step(&mut self, step: StepInfo, beats: u32) {
        self.sections.push(Section::Step { step, beats });
    }

    pub(crate) fn add_freestyle(&mut self, beats: u32) {
        self.sections.push(Section::Freestyle { beats });
    }

    /// After n beats of the lesson, get the current step and beat remainder.
    pub(crate) fn step_at_beat(&self, mut beat: u32) -> Option<(&StepInfo, u32)> {
        for section in &self.sections {
            if beat < section.beats() {
                match &section {
                    Section::Step { step, .. } => {
                        return Some((step, beat));
                    }
                    Section::Freestyle { .. } => {
                        return None;
                    }
                }
            }
            beat -= section.beats();
        }
        None
    }

    /// How far the body position has moved from the origin at a specific beat.
    pub(crate) fn pose_body_shift_at_beat(&self, mut beat: u32) -> Cartesian2d {
        let mut shift = Cartesian2d::default();
        for section in &self.sections {
            let beats_on_step = u32::min(section.beats(), beat);
            shift = shift + section.body_shift(beats_on_step);
            if beat < section.beats() {
                break;
            }
            beat -= section.beats();
        }
        shift
    }

    /// How many beats to track for
    pub(crate) fn tracked_beats(&self) -> u32 {
        self.sections.iter().map(|section| section.beats()).sum()
    }
}

impl Section {
    fn beats(&self) -> u32 {
        match self {
            Section::Step { beats, .. } => *beats,
            Section::Freestyle { beats } => *beats,
        }
    }

    fn body_shift(&self, beat: u32) -> Cartesian2d {
        match self {
            Section::Step { step, .. } => step.body_shift(beat as usize),
            Section::Freestyle { .. } => Cartesian2d::default(),
        }
    }
}
