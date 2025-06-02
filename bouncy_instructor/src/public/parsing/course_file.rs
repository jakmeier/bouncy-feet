use crate::intern::content_collection::ContentCollection;
use crate::intern::step::StepSource;
use crate::public::course::LessonPart;
use crate::public::Course;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::pose_file::Pose;
use super::step_file::Step;
use super::{ParseFileError, VersionCheck};

pub(crate) const CURRENT_VERSION: u16 = 0;

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseFile {
    version: u8,
    id: String,
    names: TranslatedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    explanations: Option<TranslatedString>,
    featured_step: String,
    video: Option<String>,
    lessons: Vec<Lesson>,
    poses: Vec<Pose>,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Lesson {
    names: TranslatedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    explanations: Option<TranslatedString>,
    /// Video to show in the preview, usually with explanations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    explainer_video: Option<String>,
    /// Video to show during the exercise, to main lessons song, in that song's pace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    front_video: Option<String>,
    /// Alternative view from the back to show during the exercise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    back_video: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    song: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    song_timestamp: Option<u64>,
    parts: Vec<Part>,
    energy: u8,
    difficulty: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Part {
    pub(crate) step: String,
    // How many times the step should be repeated.
    pub(crate) repeat: u32,
    // At what pace the step should be danced.
    pub(crate) subbeats_per_move: u8,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(transparent)]
pub(crate) struct TranslatedString {
    inner: HashMap<String, String>,
}

impl CourseFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let check: VersionCheck = ron::from_str(text)?;
        if check.version == CURRENT_VERSION {
            let parsed: CourseFile = ron::from_str(text)?;
            Ok(parsed)
        } else {
            Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: check.version,
            })
        }
    }

    pub(crate) fn into_course(self, lang: &str) -> Result<Course, ParseFileError> {
        // The course object uses its own collection of poses and steps.
        let mut collection = ContentCollection::default();
        collection.add_poses(self.poses)?;
        collection.add_steps(self.steps, StepSource::new("course".to_owned()))?;

        let name = self
            .names
            .take(lang)
            .ok_or_else(|| ParseFileError::MissingTranslation {
                id: self.id.clone(),
                lang: lang.to_owned(),
            })?;
        let explanation = self
            .explanations
            .map(|translated| {
                translated
                    .take(lang)
                    .ok_or_else(|| ParseFileError::MissingTranslation {
                        id: self.id.clone(),
                        lang: lang.to_owned(),
                    })
            })
            .transpose()?;
        let mut course = Course {
            name,
            explanation,
            id: self.id.clone(),
            featured_step_id: self.featured_step,
            video: self.video,
            lessons: vec![],
            collection,
        };

        for lesson in self.lessons {
            let validated_lesson = lesson.unpack(lang, &course.id, &course.collection)?;
            course.add_lesson(validated_lesson);
        }
        Ok(course)
    }
}

impl Lesson {
    /// Convert from a course file lesson to an internal representation,
    /// validating fields and packing fields for easier use in the application.
    pub(crate) fn unpack(
        self,
        lang: &str,
        course_id: &str,
        collection: &ContentCollection,
    ) -> Result<crate::public::course::Lesson, ParseFileError> {
        let name = self
            .names
            .take(lang)
            .ok_or_else(|| ParseFileError::MissingTranslation {
                id: format!("lesson name of {}", course_id),
                lang: lang.to_owned(),
            })?;
        let explanation = self
            .explanations
            .map(|translated| {
                translated
                    .take(lang)
                    .ok_or_else(|| ParseFileError::MissingTranslation {
                        id: format!("explanation of lesson in {}", course_id),
                        lang: lang.to_owned(),
                    })
            })
            .transpose()?;

        let parts = self
            .parts
            .into_iter()
            .map(|p| LessonPart::new(p.step, collection, p.repeat, p.subbeats_per_move))
            .collect::<Result<_, _>>()?;

        Ok(crate::public::course::Lesson {
            name,
            explanation,
            parts,
            explainer_video: self.explainer_video,
            front_video: self.front_video,
            back_video: self.back_video,
            song: self.song,
            song_timestamp: self.song_timestamp.map(|int| int as f64),
            difficulty: self.difficulty,
            energy: self.energy,
        })
    }
}

impl TranslatedString {
    pub fn take(mut self, lang: &str) -> Option<String> {
        let key = if lang.len() > 2 { &lang[0..2] } else { lang };
        let result = self
            .inner
            .remove(key)
            .or_else(|| self.inner.remove("en"))
            .or_else(|| self.inner.into_values().next());
        #[cfg(target_arch = "wasm32")]
        result.as_ref().inspect(|s| {
            wasm_bindgen::intern(s);
        });
        #[allow(clippy::let_and_return)]
        result
    }

    pub fn get(&self, lang: &str) -> Option<String> {
        let key = if lang.len() > 2 { &lang[0..2] } else { lang };
        let result = self
            .inner
            .get(key)
            .or_else(|| self.inner.get("en"))
            .or_else(|| self.inner.values().next());
        #[cfg(target_arch = "wasm32")]
        result.as_ref().inspect(|s| {
            wasm_bindgen::intern(s);
        });
        result.cloned()
    }

    pub fn set(&mut self, lang: String, name: String) {
        self.inner.insert(lang, name);
    }
}
