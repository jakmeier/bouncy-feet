use crate::dance_file;

#[derive(Clone)]
pub(crate) struct Dance {
    pub id: String,
    pub step_ids: Vec<String>,
    pub flip_orientation: Vec<bool>,
}

impl From<dance_file::Dance> for Dance {
    fn from(def: dance_file::Dance) -> Self {
        let (step_ids, flip_orientation) = def
            .steps
            .into_iter()
            .map(|step| (step.id, step.flip_orientation))
            .unzip();

        Dance {
            id: def.id,
            step_ids,
            flip_orientation,
        }
    }
}
