use crate::public::course::LessonPart;
use crate::public::Course;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ParseFileError, VersionCheck};

pub(crate) const CURRENT_VERSION: u16 = 0;

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseFile {
    version: u8,
    id: String,
    names: TranslatedString,
    featured_step: String,
    lessons: Vec<Lesson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Lesson {
    names: TranslatedString,
    icon: String,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Part {
    step: String,
    bpms: Vec<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
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
        let lessons = self
            .lessons
            .into_iter()
            .map(|lesson| {
                Ok::<_, ParseFileError>(crate::public::course::Lesson {
                    name: lesson.names.take(lang).ok_or_else(|| {
                        ParseFileError::MissingTranslation {
                            id: format!("lesson of {}", self.id.clone()),
                            lang: lang.to_owned(),
                        }
                    })?,
                    icon: lesson.icon,
                    parts: lesson
                        .parts
                        .into_iter()
                        .map(|p| LessonPart {
                            step: p.step,
                            bpms: p.bpms,
                        })
                        .collect(),
                })
            })
            .collect::<Result<Vec<_>, ParseFileError>>()?;
        let course = Course {
            name: self
                .names
                .take(lang)
                .ok_or_else(|| ParseFileError::MissingTranslation {
                    id: self.id.clone(),
                    lang: lang.to_owned(),
                })?,
            id: self.id,
            featured_step_id: self.featured_step,
            lessons,
        };
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
