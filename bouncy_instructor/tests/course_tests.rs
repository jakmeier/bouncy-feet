use bouncy_instructor::parse_course_str;
use bouncy_instructor::Cartesian2d;

mod common;

// New courses must be added manually here to be checked
const COURSES: [(&str, &str); 6] = [
    (
        "000-rm-basics",
        include_str!("./data/courses/000-rm-basics.ron"),
    ),
    (
        "002-v-step-basics",
        include_str!("./data/courses/002-v-step-basics.ron"),
    ),
    ("003-intro", include_str!("./data/courses/003-intro.ron")),
    (
        "004-rm-practice",
        include_str!("./data/courses/004-rm-practice.ron"),
    ),
    ("005-rrm-basics", include_str!("./data/courses/005-rrm.ron")),
    ("006-dnb", include_str!("./data/courses/006-dnb.ron")),
];

#[test]
fn test_courses_parse_en() {
    check_courses_parse("en");
}

#[test]
fn test_courses_parse_de() {
    check_courses_parse("de");
}

fn check_courses_parse(lang: &str) {
    for (id, course) in COURSES {
        parse_course_str(course, lang).unwrap_or_else(|err| {
            panic!("static course {id} could not be parsed with language {lang}. {err}")
        });
    }
}

/// Ensure the instructor does not leave the visible area.
#[test]
fn test_courses_in_boundary() {
    //note:  min and max might need better fine-tuning
    let min = Cartesian2d::new(-0.3, -0.5);
    let max = Cartesian2d::new(0.3, 0.5);
    for (id, course) in COURSES {
        println!("checking course {id} is in boundary");
        check_course_in_boundary(course, min, max);
    }
}

fn check_course_in_boundary(course_str: &str, min: Cartesian2d, max: Cartesian2d) {
    let lang = "en";

    let course = parse_course_str(course_str, lang).expect("static course must parse fine");

    for lesson_index in 0..course.lessons().len() {
        let tracker = course
            .tracker(lesson_index)
            .expect("tracker for every lesson must be available");

        for subbeat in 0..tracker.tracked_subbeats() as i32 {
            let cursor = tracker.cursor_at_subbeat(subbeat, false);
            let body_shift = tracker.pose_body_shift(&cursor);

            if body_shift.x < min.x
                || body_shift.y < min.y
                || body_shift.x > max.x
                || body_shift.y > max.y
            {
                panic!("body_shift is {body_shift:?} at lesson {lesson_index} subbeat {subbeat} which is outside the acceptable boundary of {min:?} - {max:?}");
            }

            // we could also check that the skeleton is in boundary, which would perhaps be more accurate
            // for now, just consruct a skeleton to ensure it doesn't crash because no other test currently covers this
            let _skeleton = tracker.pose_skeleton_at(&cursor);
        }
    }
}
