use super::course_file::TranslatedString;
use super::pose_file::Pose;
use super::step_file::Step;
use super::video_def::{Marker, VideoDef};
use super::{ParseFileError, VersionCheck};
use crate::intern::content_collection::ContentCollection;
use crate::intern::step::StepSource;
use crate::public::challenge::{Challenge, ChallengePart, ChallengeSet};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub(crate) const CURRENT_VERSION: u16 = 0;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeFile {
    version: u16,
    definition_version: u16,
    id: String,
    names: TranslatedString,
    #[serde(default)]
    explanations: Option<TranslatedString>,
    challenges: Vec<ChallengeDef>,
    #[serde(default)]
    poses: Vec<Pose>,
    #[serde(default)]
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeDef {
    id: String,
    names: TranslatedString,
    #[serde(default)]
    explanations: Option<TranslatedString>,
    #[serde(default)]
    prerequisites: Vec<String>,
    bpm: u16,
    solution_video: Option<VideoDef>,
    solution_start_timestamp: Option<u32>,
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    step: String,
    repeat: u32,
    subbeats_per_move: u8,
}

impl ChallengeFile {
    pub(crate) fn from_str(text: &str) -> Result<Self, ParseFileError> {
        let check: VersionCheck = ron::from_str(text)?;
        if check.version != CURRENT_VERSION {
            return Err(ParseFileError::VersionMismatch {
                expected: CURRENT_VERSION,
                found: check.version,
            });
        }
        Ok(ron::from_str(text)?)
    }

    pub(crate) fn into_challenge_set(self, lang: &str) -> Result<ChallengeSet, ParseFileError> {
        let mut collection = ContentCollection::default();
        collection.add_poses(self.poses)?;
        collection.add_steps(self.steps, StepSource::new("challenge".to_owned()))?;
        let set_id = self.id.clone();
        let name = translation(self.names, lang, &set_id)?;
        let explanation = self
            .explanations
            .map(|v| translation(v, lang, &set_id))
            .transpose()?;
        let ids: HashSet<String> = self.challenges.iter().map(|c| c.id.clone()).collect();
        if ids.len() != self.challenges.len() {
            return Err(ParseFileError::DuplicateChallengeId);
        }
        let mut challenges = Vec::with_capacity(self.challenges.len());
        for challenge in self.challenges {
            if challenge.bpm == 0 {
                return Err(ParseFileError::InvalidChallengeValue("bpm"));
            }
            if challenge.parts.is_empty() {
                return Err(ParseFileError::EmptyChallengeCombo(challenge.id));
            }
            let video = challenge
                .solution_video
                .ok_or_else(|| ParseFileError::MissingSolutionVideo(challenge.id.clone()))?;
            validate_video(&challenge.id, &video, challenge.solution_start_timestamp)?;
            for prerequisite in &challenge.prerequisites {
                if prerequisite == &challenge.id {
                    return Err(ParseFileError::ChallengeSelfDependency(challenge.id));
                }
                if !ids.contains(prerequisite) {
                    return Err(ParseFileError::UnknownChallengePrerequisite(
                        prerequisite.clone(),
                    ));
                }
            }
            let mut parts = vec![];
            for p in challenge.parts {
                if p.repeat == 0 || p.subbeats_per_move == 0 {
                    return Err(ParseFileError::InvalidChallengeValue(
                        "repeat or subbeats_per_move",
                    ));
                }
                parts.push(ChallengePart::new(
                    p.step,
                    &collection,
                    p.repeat,
                    p.subbeats_per_move,
                )?);
            }
            challenges.push(Challenge {
                id: challenge.id.clone(),
                name: translation(challenge.names, lang, &challenge.id)?,
                explanation: challenge
                    .explanations
                    .map(|v| translation(v, lang, &challenge.id))
                    .transpose()?,
                prerequisites: challenge.prerequisites,
                bpm: challenge.bpm,
                solution_video: video.into(),
                solution_start_timestamp: challenge.solution_start_timestamp.unwrap() as f64,
                parts,
            });
        }
        validate_graph(&challenges)?;
        Ok(ChallengeSet {
            id: set_id,
            definition_version: self.definition_version,
            name,
            explanation,
            challenges,
            collection,
        })
    }
}
fn translation(value: TranslatedString, lang: &str, id: &str) -> Result<String, ParseFileError> {
    value
        .take(lang)
        .ok_or_else(|| ParseFileError::MissingTranslation {
            id: id.into(),
            lang: lang.into(),
        })
}
fn validate_video(id: &str, video: &VideoDef, start: Option<u32>) -> Result<(), ParseFileError> {
    let VideoDef::Full {
        path,
        beats,
        markers,
    } = video
    else {
        return Err(ParseFileError::InvalidSolutionVideoAlignment(id.into()));
    };
    if path.is_empty()
        || beats.is_empty()
        || start.is_none()
        || !markers.values().any(|m| matches!(m, Marker::Start))
    {
        return Err(ParseFileError::InvalidSolutionVideoAlignment(id.into()));
    }
    Ok(())
}
fn validate_graph(challenges: &[Challenge]) -> Result<(), ParseFileError> {
    if challenges.is_empty() {
        return Err(ParseFileError::NoChallengeRoot);
    }
    let by_id: HashMap<_, _> = challenges.iter().map(|c| (c.id.as_str(), c)).collect();
    let roots: Vec<_> = challenges
        .iter()
        .filter(|c| c.prerequisites.is_empty())
        .collect();
    if roots.is_empty() {
        return Err(ParseFileError::NoChallengeRoot);
    }
    fn visit<'a>(
        id: &'a str,
        by_id: &HashMap<&'a str, &'a Challenge>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
    ) -> Result<(), ParseFileError> {
        if visited.contains(id) {
            return Ok(());
        }
        if !visiting.insert(id) {
            return Err(ParseFileError::ChallengeDependencyCycle);
        }
        for p in &by_id[id].prerequisites {
            visit(p, by_id, visiting, visited)?;
        }
        visiting.remove(id);
        visited.insert(id);
        Ok(())
    }
    let mut visited = HashSet::new();
    for c in challenges {
        visit(&c.id, &by_id, &mut HashSet::new(), &mut visited)?;
    }
    Ok(())
}
