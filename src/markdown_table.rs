pub fn to_tsv(md: &str) -> String {
    parse_markdown_table(md)
        .iter()
        .map(|row| row.join("\t"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_markdown_table(md: &str) -> Vec<Vec<String>> {
    md.lines()
        .map(str::trim)
        .filter(|line| is_data_line(line))
        .map(|line| parse_md_row(line))
        .collect()
}

/// テーブルの「データ行」かどうか判定
/// - `|---|---|` のような区切り行は false
/// - `|` で始まらない行も false
fn is_data_line(line: &str) -> bool {
    // '|' で始まらずデータ行ではない
    if !line.starts_with('|') {
        return false;
    }
    // 区切り行は全て '|', '-', ' ' のみで構成される
    let is_separator = line.chars().all(|c| matches!(c, '|' | '-' | ' '));
    !is_separator
}

/// 1 行分の Markdown テーブルを Vec<String> に分割
fn parse_md_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}
