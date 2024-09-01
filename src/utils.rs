pub fn box_(text: impl Into<String>) -> String {
    let text: String = text.into();
    let tl = "╭";
    let tr = "╮";
    let bl = "╰";
    let br = "╯";
    let h = "─";
    let v = "│";
    let pad = " ";
    let mut h_line = h.to_owned();
    let text_seg: Vec<&str> = text
        .split("\n")
        .collect();
    let mut text_seg: Vec<String> = text_seg
        .iter()
        .map(|el| el.to_string())
        .collect();
    let mut max_len = text_seg[0].len();
    let mut paddings = Vec::<String>::new();
    let mut out;
    
    for seg in text_seg.iter() {
        if max_len < seg.len() {
            max_len = seg.len();
        }
    }

    for _ in 0..max_len {
        h_line += h;
    }

    for seg in text_seg.iter_mut() {
        let mut r_pad = pad.to_owned();
        let diff = max_len
            .checked_sub(seg.len())
            .unwrap_or(0);

        for _ in 0..diff {
            r_pad += pad;
        }
        paddings.push(r_pad);
    }
    h_line += h;

    out = format!("{}{}{}\n", 
        tl,h_line,tr,
    );

    for (i, seg) in text_seg.iter().enumerate() {
        out += &format!("{v}{pad}{seg}{}{v}\n", paddings[i]);
    }

    out += &format!("{}{}{}\n", 
        bl,h_line,br
    );

    return out;
}

