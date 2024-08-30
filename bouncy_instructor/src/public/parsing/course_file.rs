use crate::intern::dance_collection::DanceCollection;
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
    lessons: Vec<Lesson>,
    poses: Vec<Pose>,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Lesson {
    names: TranslatedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    explanations: Option<TranslatedString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    video: Option<String>,
    icon: String,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Part {
    pub(crate) step: String,
    pub(crate) bpms: Vec<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) explanations: Option<TranslatedString>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
        let mut collection = DanceCollection::default();
        collection.add_poses(self.poses)?;
        collection.add_steps(&self.steps, "course".to_owned())?;

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
            lessons: vec![],
            collection,
        };

        for lesson in self.lessons {
            let name =
                lesson
                    .names
                    .take(lang)
                    .ok_or_else(|| ParseFileError::MissingTranslation {
                        id: format!("lesson name of {}", self.id.clone()),
                        lang: lang.to_owned(),
                    })?;
            let lesson_explanation = lesson
                .explanations
                .map(|translated| {
                    translated
                        .take(lang)
                        .ok_or_else(|| ParseFileError::MissingTranslation {
                            id: format!("explanation of lesson in {}", self.id.clone()),
                            lang: lang.to_owned(),
                        })
                })
                .transpose()?;
            course.add_lesson(
                name,
                lesson_explanation,
                lesson.video,
                lesson.icon,
                lesson.parts,
                lang,
            )?;
        }
        Ok(course)
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
}
