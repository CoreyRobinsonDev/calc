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

pub fn bin_dec(binary: impl Into<String>) -> i32 {
    let binary: String = binary.into();
    let mut num: i32 = 0;

    for (i, dig) in binary.chars().rev().enumerate() {
        if dig == '1' {num += 2i32.pow(i as u32)}
    }

    return num;
}

pub fn hex_dec(hex: impl Into<String>) -> i32 {
    let hex: String = hex.into();
    let mut num: i32 = 0;

    for (i, dig) in hex.chars().rev().enumerate() {
        match dig {
            '1' => num += 1 * 16i32.pow(i as u32),
            '2' => num += 2 * 16i32.pow(i as u32),
            '3' => num += 3 * 16i32.pow(i as u32),
            '4' => num += 4 * 16i32.pow(i as u32),
            '5' => num += 5 * 16i32.pow(i as u32),
            '6' => num += 6 * 16i32.pow(i as u32),
            '7' => num += 7 * 16i32.pow(i as u32),
            '8' => num += 8 * 16i32.pow(i as u32),
            '9' => num += 9 * 16i32.pow(i as u32),
            'a' | 'A' => num += 10 * 16i32.pow(i as u32),
            'b' | 'B' => num += 11 * 16i32.pow(i as u32),
            'c' | 'C' => num += 12 * 16i32.pow(i as u32),
            'd' | 'D' => num += 13 * 16i32.pow(i as u32),
            'e' | 'E' => num += 14 * 16i32.pow(i as u32),
            'f' | 'F' => num += 15 * 16i32.pow(i as u32),
            _ => {}
        }
    }

    return num;
}

pub fn calc(equation: impl Into<String>) -> Vec<String> {
    let equation: String = equation.into();
    let ops = "%**/+-&|^";
    let mut a: String = String::new();
    let mut b: String = String::new();
    let mut op: String = String::new();
    let mut eq: i32 = 0;

    for ch in equation.chars() {
        if ch == '\n' {break}
        if ops.contains(ch) {
            op += &ch.to_string();
            continue;
        }
        if op.len() != 0 {
            b += &ch.to_string();
        } else {
            a += &ch.to_string();
        }
    }
    let a: i32 = if a.contains('b') {
        bin_dec(a)
    } else if a.contains('x') {
        hex_dec(a)
    } else {
        a.parse().unwrap_or(0)
    };
    let b: i32 = if b.contains('b') {
        bin_dec(b)
    } else if b.contains('x') {
        hex_dec(b)
    } else {
        b.parse().unwrap_or(0)
    };

    match op.as_str() {
        "+" => eq = a.checked_add(b).unwrap_or(i32::MAX),
        "-" => eq = a.checked_sub(b).unwrap_or(i32::MIN),
        "/" => eq = a.checked_div(b).unwrap_or(0),
        "*" => eq = a.checked_mul(b).unwrap_or(i32::MAX),
        "**" => eq = a.checked_pow(b as u32).unwrap_or(i32::MAX),
        "%" => eq = a % b,
        "^" => eq = a ^ b,
        "|" => eq = a | b,
        "&" => eq = a & b,
        _ => {}
    }

    return vec![a.to_string(),op,b.to_string(),eq.to_string()];
}
