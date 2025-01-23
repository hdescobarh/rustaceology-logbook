use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut buffer = String::new();

    list.windows(2).for_each(|w| {
        writeln!(&mut buffer, "For want of a {} the {} was lost.", w[0], w[1]).unwrap()
    });

    list.first()
        .and_then(|s| write!(&mut buffer, "And all for the want of a {s}.").ok());

    buffer
}
