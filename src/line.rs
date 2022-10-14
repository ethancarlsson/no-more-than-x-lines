struct LinesChanged {
    file: String,
    from: int32,
    to: int32,
}

pub fn get_changed_lines_per_file(diff: String) -> LinesChanged {
}
