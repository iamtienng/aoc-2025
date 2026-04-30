use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
struct Shape {
    cells: Vec<(i32, i32)>,
    area: usize,
    orientations: Vec<(i32, i32)>, // (width, height) for unique orientations
}

fn normalize_cells(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if cells.is_empty() {
        return vec![];
    }
    let min_x = cells.iter().map(|c| c.0).min().unwrap();
    let min_y = cells.iter().map(|c| c.1).min().unwrap();
    let mut v: Vec<(i32, i32)> = cells.iter().map(|&(x, y)| (x - min_x, y - min_y)).collect();
    v.sort_unstable();
    v
}

fn all_transforms(cells: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    // generate rotations (0,90,180,270) and flips (none, flip-x)
    let mut res: Vec<Vec<(i32, i32)>> = Vec::new();
    for &flip in &[false, true] {
        let current: Vec<(i32, i32)> = if flip {
            cells.iter().map(|&(x, y)| (-x, y)).collect()
        } else {
            cells.to_vec()
        };
        let mut cur = current.clone();
        for _ in 0..4 {
            let normalized = normalize_cells(&cur);
            if !res.iter().any(|r| *r == normalized) {
                res.push(normalized);
            }
            // rotate 90 degrees: (x,y) -> (-y, x)
            cur = cur.iter().map(|&(x, y)| (-y, x)).collect();
        }
    }
    res
}

fn is_region_header(line: &str) -> bool {
    // crude but robust check: <digits>x<digits>:
    let s = line.trim();
    if s.is_empty() {
        return false;
    }
    if let Some(colpos) = s.find(':') {
        let left = s[..colpos].trim();
        if let Some(xpos) = left.find('x') {
            let w = &left[..xpos];
            let h = &left[xpos + 1..];
            return !w.is_empty()
                && !h.is_empty()
                && w.chars().all(|c| c.is_ascii_digit())
                && h.chars().all(|c| c.is_ascii_digit());
        }
    }
    false
}

fn parse_input(text: &str) -> (Vec<Shape>, Vec<(i32, i32, Vec<usize>)>) {
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end()).collect();

    // find first region header line index
    let mut region_start: Option<usize> = None;
    for (i, &line) in lines.iter().enumerate() {
        if is_region_header(line) {
            region_start = Some(i);
            break;
        }
    }

    let region_idx = match region_start {
        Some(i) => i,
        None => lines.len(),
    };

    // parse shapes from lines[0..region_idx)
    let mut shapes_map: std::collections::BTreeMap<usize, Vec<String>> = Default::default();
    let mut i = 0usize;
    while i < region_idx {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        // expect "N:" where N is integer
        if let Some(colpos) = line.find(':') {
            let left = line[..colpos].trim();
            if left.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(idx_num) = left.parse::<usize>() {
                    // collect subsequent non-empty lines until blank or end or next digit-colon line
                    i += 1;
                    let mut rows: Vec<String> = Vec::new();
                    while i < region_idx {
                        let l = lines[i];
                        if l.trim().is_empty() {
                            break;
                        }
                        // if next shape header encountered, stop
                        if let Some(pcol) = l.find(':') {
                            let lleft = l[..pcol].trim();
                            if lleft.chars().all(|c| c.is_ascii_digit()) && l.trim().ends_with(':')
                            {
                                break;
                            }
                        }
                        rows.push(l.to_string());
                        i += 1;
                    }
                    shapes_map.insert(idx_num, rows);
                    continue;
                }
            }
        }
        // skip unknown line
        i += 1;
    }

    // convert shapes_map to vec indexed by shape index
    let max_idx = shapes_map.keys().cloned().max().unwrap_or(0);
    let mut shapes_vec: Vec<Shape> = Vec::new();
    for idx in 0..=max_idx {
        if let Some(rows) = shapes_map.get(&idx) {
            let mut cells: Vec<(i32, i32)> = Vec::new();
            for (y, row) in rows.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == '#' {
                        cells.push((x as i32, y as i32));
                    }
                }
            }
            let area = cells.len();
            let transforms = all_transforms(&cells);
            let mut orientations: Vec<(i32, i32)> = Vec::new();
            for t in transforms {
                if t.is_empty() {
                    continue;
                }
                let min_x = t.iter().map(|c| c.0).min().unwrap();
                let max_x = t.iter().map(|c| c.0).max().unwrap();
                let min_y = t.iter().map(|c| c.1).min().unwrap();
                let max_y = t.iter().map(|c| c.1).max().unwrap();
                let w = max_x - min_x + 1;
                let h = max_y - min_y + 1;
                if !orientations.contains(&(w, h)) {
                    orientations.push((w, h));
                }
            }
            shapes_vec.push(Shape {
                cells,
                area,
                orientations,
            });
        } else {
            shapes_vec.push(Shape {
                cells: Vec::new(),
                area: 0,
                orientations: Vec::new(),
            });
        }
    }

    // parse regions from lines[region_idx..]
    let mut regions: Vec<(i32, i32, Vec<usize>)> = Vec::new();
    let mut j = region_idx;
    while j < lines.len() {
        let line = lines[j].trim();
        j += 1;
        if line.is_empty() {
            continue;
        }
        if let Some(colpos) = line.find(':') {
            let dim = line[..colpos].trim();
            let rest = line[colpos + 1..].trim();
            if let Some(xpos) = dim.find('x') {
                let wstr = &dim[..xpos];
                let hstr = &dim[xpos + 1..];
                if let (Ok(w), Ok(h)) = (wstr.parse::<i32>(), hstr.parse::<i32>()) {
                    let counts: Vec<usize> = rest
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    regions.push((w, h, counts));
                }
            }
        }
    }

    (shapes_vec, regions)
}

pub fn part_one() {
    let input_path = Path::new("../input.txt");
    let text = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read ../input.txt: {}", e);
            let _ = fs::write("../output_part_one.txt", "0\n");
            return;
        }
    };

    let (shapes, regions) = parse_input(&text);

    // quick sanity: if no regions parsed, warn
    if regions.is_empty() {
        eprintln!("Warning: no regions parsed from input. Check input format.");
    }

    let mut fitable_count: usize = 0usize;

    'region_loop: for (w, h, counts) in regions {
        let mut total_needed: usize = 0;
        for (i, &cnt) in counts.iter().enumerate() {
            let area = shapes.get(i).map(|s| s.area).unwrap_or(0);
            total_needed += area * cnt;
        }
        let region_area = (w as usize) * (h as usize);
        if total_needed > region_area {
            continue 'region_loop;
        }

        for (i, &cnt) in counts.iter().enumerate() {
            if cnt == 0 {
                continue;
            }
            let s = match shapes.get(i) {
                Some(s) => s,
                None => continue 'region_loop,
            };
            if s.area == 0 {
                continue 'region_loop;
            }
            let mut any_fit = false;
            for &(ow, oh) in &s.orientations {
                if ow <= w && oh <= h {
                    any_fit = true;
                    break;
                }
            }
            if !any_fit {
                continue 'region_loop;
            }
        }

        fitable_count += 1;
    }

    let out_path = "../output_part_one.txt";
    match fs::File::create(out_path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", fitable_count) {
                eprintln!("Failed to write output: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create {}: {}", out_path, e);
        }
    }
}

fn main() {
    part_one();
}
