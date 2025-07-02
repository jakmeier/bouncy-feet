use std::collections::BTreeMap;

/// Define a video to display in frontend in one of several ways.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum VideoDef {
    /// Defined with additional meta data
    Full {
        path: String,
        beats: Vec<u32>,
        markers: BTreeMap<u32, Marker>,
    },
    /// Defined by just the path
    Simple(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Marker {
    Start,
    Step(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ron::de::from_str;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct TestContainer {
        video: VideoDef,
    }

    #[test]
    fn test_parse_full_video() {
        let ron_data = r#"
            (
                video: (
                    path: "https://example.com/video.mp4",
                    beats: [1000, 2000, 3000],
                    markers: {
                        1000: Start,
                        2000: Step("jump"),
                    }
                )
            )
        "#;

        let TestContainer { video } = from_str(ron_data).expect("Failed to parse full video");
        let VideoDef::Full {
            path,
            beats,
            markers,
        } = video
        else {
            panic!("Expected full video variant")
        };

        assert_eq!(path, "https://example.com/video.mp4");
        assert_eq!(beats, vec![1000, 2000, 3000]);
        assert_eq!(markers[&1000], Marker::Start);
        assert_eq!(markers[&2000], Marker::Step("jump".to_string()));
    }

    #[test]
    fn test_parse_simple_video() {
        let ron_data = r#"( video: "https://example.com/simple.mp4" )"#;

        let TestContainer { video } = from_str(ron_data).expect("Failed to parse simple video");
        let VideoDef::Simple(url) = video else {
            panic!("Expected simple video variant")
        };

        assert_eq!(url, "https://example.com/simple.mp4");
    }

    #[test]
    fn test_parse_marker() {
        assert_eq!(Marker::Start, ron::from_str("Start").unwrap());
        assert_eq!(
            Marker::Step("jump".to_owned()),
            ron::from_str(r#"Step("jump")"#).unwrap()
        );

        assert_eq!(
            vec![(7u32, Marker::Step("xyz".to_owned()))],
            ron::from_str::<Vec<(u32, Marker)>>(r#"[(7,Step("xyz"))]"#).unwrap()
        );
    }
}
