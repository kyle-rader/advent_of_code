fn main() {
    let maxes = [
        ("i8", i8::MAX as u128),
        ("u8", u8::MAX as u128),
        ("i16", i16::MAX as u128),
        ("u16", u16::MAX as u128),
        ("u32", u32::MAX as u128),
        ("i32", i32::MAX as u128),
        ("i64", i64::MAX as u128),
        ("u64", u64::MAX as u128),
        ("isize", isize::MAX as u128),
        ("usize", usize::MAX as u128),
        ("i128", i128::MAX as u128),
        ("u128", u128::MAX),
        ("f32", f32::MAX as u128),
        ("f64", f64::MAX as u128),
    ];

    print_max(&maxes);
}

fn print_max(maxes: &[(&str, u128)]) {
    let value_width = maxes
        .iter()
        .map(|(_, val)| val.to_string().len())
        .max()
        .unwrap();

    let name_width = maxes.iter().map(|(name, _)| name.len()).max().unwrap();

    for (name, max) in maxes.iter() {
        println!(
            "max {:name_width$}: {:value_width$}",
            name,
            max,
            name_width = name_width,
            value_width = value_width
        );
    }
}
