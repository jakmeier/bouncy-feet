use bouncy_instructor::parse_course_str;

mod common;

#[test]
fn test_courses_parse_en() {
    test_courses_parse("en");
}

#[test]
fn test_courses_parse_de() {
    test_courses_parse("de");
}

fn test_courses_parse(lang: &str) {
    parse_course_str(include_str!("./data/courses/000-rm-basics.ron"), lang)
        .expect("static course must parse fine");
    parse_course_str(include_str!("./data/courses/002-v-step-basics.ron"), lang)
        .expect("static course must parse fine");
    parse_course_str(include_str!("./data/courses/003-intro.ron"), lang)
        .expect("static course must parse fine");
}
