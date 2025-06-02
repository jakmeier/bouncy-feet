use super::body_shift::BodyShift;
use super::step_pace::StepPace;
use crate::skeleton::Cartesian2d;
use crate::tracker::{DanceCursor, TeacherView};
use crate::StepInfo;

/// Dynamically switch between steps, switch between views for showing the next
/// step and going back to full camera mode for dancers to see themselves.
#[derive(Default)]
pub(crate) struct Teacher {
    sections: Vec<Section>,
    total_subbeats: u32,
    body_shift: BodyShift,
    teacher_display_mode: TeacherDisplayMode,
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

/// What to display while the user dances.
#[derive(Default)]
enum TeacherDisplayMode {
    /// Show the user themself while they need to dance, maybe with
    #[default]
    Mirror,
    /// Show the teacher video while the user dances
    Video,
}

impl Teacher {
    /// Show a video of the teacher instead of a animation.
    pub(crate) fn use_video(&mut self, yes: bool) {
        if yes {
            self.teacher_display_mode = TeacherDisplayMode::Video;
        } else {
            self.teacher_display_mode = TeacherDisplayMode::Mirror;
        }
    }

    /// A step to dance by the student.
    pub(crate) fn add_step(&mut self, step: StepInfo, repeat: u32, pace: StepPace) {
        let beats = repeat * step.num_poses() as u32 * pace.subbeats_per_pose() / 2;

        assert_ne!(beats, 0);
        for _ in 0..repeat {
            self.body_shift.add(step.body_shift.clone());
        }
        self.sections.push(Section::Step(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
        self.update_tracked_subbeats();
    }

    /// The instructor shows the step, the student can watch.
    pub(crate) fn show_step(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::ShowStep(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
        self.update_tracked_subbeats();
    }

    /// Any move allowed from the student.
    pub(crate) fn add_freestyle(&mut self, beats: u32) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Freestyle {
            subbeats: beats * 2,
        });
        self.update_tracked_subbeats();
    }

    /// Show a step to copy by the student but be more lenient in tracking.
    pub(crate) fn add_warmup(&mut self, step: StepInfo, beats: u32, pace: StepPace) {
        assert_ne!(beats, 0);
        self.sections.push(Section::Warmup(StepSection {
            step,
            subbeats: beats * 2,
            pace,
        }));
        self.update_tracked_subbeats();
    }

    pub(crate) fn step(&self, cursor: &DanceCursor) -> Option<&StepInfo> {
        self.section(cursor)
            .and_then(Section::step)
            .map(|StepSection { step, .. }| step)
    }

    /// After n subbeats of the lesson, get the current step and beat remainder.
    ///
    /// Full beat: only count 1,2,3,4 (used for bpm calculation)
    /// Sub beat: also count the "and" between
    pub(crate) fn step_at_subbeat(&self, subbeat: u32) -> Option<(&StepInfo, u32)> {
        self.section_at_subbeat(subbeat)
            .and_then(Section::step)
            .map(|StepSection { step, pace, .. }| (step, pace.pose_at_subbeat(subbeat)))
    }

    pub(crate) fn pose_duration(&self, cursor: &DanceCursor) -> Option<u32> {
        self.section(cursor)
            .and_then(|section| section.pose_duration())
    }

    pub(crate) fn cursor_at_subbeat(&self, subbeat: u32) -> DanceCursor {
        let (section_index, remainder) = self.index_at_subbeat(subbeat);
        let section = self.sections.get(section_index);
        let pose_index = section
            .and_then(|section| {
                section
                    .step()
                    .map(|StepSection { pace, .. }| pace.pose_at_subbeat(remainder as u32) as usize)
            })
            .unwrap_or(0);

        // Also calculate the #step for the frontend to use.
        let steps_current_section = self
            .sections
            .get(section_index)
            .map(|sec| remainder / sec.single_step_subbeats() as usize)
            .unwrap_or(0);
        let steps_before: usize = self.sections[..section_index]
            .iter()
            .map(|sec| sec.num_steps() as usize)
            .sum();
        let step_index = steps_current_section + steps_before;

        DanceCursor {
            subbeat,
            section_index,
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
        self.sections.get(cursor.section_index)
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
    pub(crate) fn pose_body_shift_at_subbeat(&self, subbeat: u32) -> Cartesian2d {
        let pose = self.pose_nr_subbeat(subbeat);
        self.body_shift.after_pose(pose)
    }

    /// How many poses were done after a number of subbeats
    pub(crate) fn pose_nr_subbeat(&self, mut subbeat: u32) -> usize {
        let mut poses = 0;
        let cursor = self.cursor_at_subbeat(subbeat);
        for section in &self.sections[0..cursor.section_index] {
            let section_duration = section.subbeats();
            poses += section_duration / section.pose_duration().unwrap_or(1);
            subbeat -= section_duration;
        }

        if let Some(section) = self.section(&cursor) {
            poses += subbeat / section.pose_duration().unwrap_or(1);
        }
        poses as usize
    }

    /// Whether at the given subbeat, the student should be tracked.
    pub(crate) fn is_tracking_at_subbeat(&self, subbeat: u32) -> bool {
        self.section_at_subbeat(subbeat)
            .map(Section::is_tracked)
            .unwrap_or_default()
    }

    /// What information the UI should show.
    pub(crate) fn ui_view_at_subbeat(&self, subbeat: u32) -> TeacherView {
        self.section_at_subbeat(subbeat)
            .map(|section| match &section {
                Section::Step(_) => match self.teacher_display_mode {
                    TeacherDisplayMode::Mirror => TeacherView::InstructorAndCamera,
                    TeacherDisplayMode::Video => TeacherView::TeacherVideo,
                },
                Section::Freestyle { .. } => TeacherView::UserCameraWithTracking,
                Section::Warmup(_) | Section::ShowStep(_) => match self.teacher_display_mode {
                    TeacherDisplayMode::Mirror => TeacherView::InstructorOnly,
                    TeacherDisplayMode::Video => TeacherView::TeacherVideo,
                },
            })
            .unwrap_or(TeacherView::Off)
    }

    /// The teacher is done when there is no more to show or track.
    pub(crate) fn is_done(&self, subbeat: u32) -> bool {
        subbeat >= self.tracked_subbeats()
    }

    fn update_tracked_subbeats(&mut self) {
        self.total_subbeats = self.sections.iter().map(|section| section.subbeats()).sum();
    }

    /// How many beats to track for in total
    pub(crate) fn tracked_subbeats(&self) -> u32 {
        self.total_subbeats
    }

    /// number of subbeats to delay
    pub(crate) fn subbeats_before_tracking(&self) -> u32 {
        let mut delay = 0;
        for section in &self.sections {
            if section.is_tracked() {
                break;
            }
            delay += section.subbeats();
        }
        delay
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

    fn single_step_subbeats(&self) -> u32 {
        match self {
            Section::Step(step_section)
            | Section::ShowStep(step_section)
            | Section::Warmup(step_section) => {
                let StepSection { step, pace, .. } = step_section;
                pace.subbeats_per_pose() * step.num_poses() as u32
            }
            // TODO: This doesn't quite make sense
            Section::Freestyle { subbeats } => *subbeats,
        }
    }

    fn num_steps(&self) -> u32 {
        let per_step = self.single_step_subbeats();
        self.subbeats().div_ceil(per_step)
    }

    fn step(&self) -> Option<&StepSection> {
        match &self {
            Section::Step(step_section)
            | Section::ShowStep(step_section)
            | Section::Warmup(step_section) => Some(step_section),
            Section::Freestyle { .. } => None,
        }
    }

    fn pose_duration(&self) -> Option<u32> {
        self.step()
            .map(|StepSection { pace, .. }| pace.subbeats_per_pose())
    }

    fn is_tracked(&self) -> bool {
        match self {
            Section::Step(_) | Section::Freestyle { .. } | Section::Warmup(_) => true,
            Section::ShowStep(_) => false,
        }
    }
}
