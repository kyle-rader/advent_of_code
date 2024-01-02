fn main() {
    let maxes = [
        ("i8", std::i8::MAX as u128),
        ("u8", std::u8::MAX as u128),
        ("i16", std::i16::MAX as u128),
        ("u16", std::u16::MAX as u128),
        ("u32", std::u32::MAX as u128),
        ("i32", std::i32::MAX as u128),
        ("i64", std::i64::MAX as u128),
        ("u64", std::u64::MAX as u128),
        ("isize", std::isize::MAX as u128),
        ("usize", std::usize::MAX as u128),
        ("i128", std::i128::MAX as u128),
        ("u128", std::u128::MAX),
        ("f32", std::f32::MAX as u128),
        ("f64", std::f64::MAX as u128),
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
