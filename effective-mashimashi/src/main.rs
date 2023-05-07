fn main() {
    println!("| ヤサイ | ニンニク | アブラ | 呪文 | 長さ |");
    println!("|---|---|---|---|---|");
    for yasai in 0..5 {
        for ninniku in 0..5 {
            for abura in 0..5 {
                let mut res = spell(0, yasai) + &spell(1, ninniku) + &spell(2, abura);
                for i in 0..5 {
                    if i == 3 {
                        continue;
                    }
                    let mut tmp = all_spell(i);
                    if yasai != i {
                        tmp += &additional_spell(0, yasai);
                    }
                    if ninniku != i {
                        tmp += &additional_spell(1, ninniku);
                    }
                    if abura != i {
                        tmp += &additional_spell(2, abura);
                    }
                    if tmp.len() < res.len() {
                        res = tmp;
                    }
                }
                if res.len() == 0 {
                    res = "ソノママ".to_string();
                }
                println!(
                    "|{}|{}|{}|{}|{}|",
                    yasai,
                    ninniku,
                    abura,
                    res,
                    res.chars().count()
                );
            }
        }
    }
}

fn spell(i: usize, j: usize) -> String {
    if j == 2 {
        return "".to_string();
    }
    let mut res = match i {
        0 => "ヤサイ",
        1 => "ニンニク",
        _ => "アブラ",
    }
    .to_string();
    res += match j {
        0 => "ナシ",
        1 => "スクナメ",
        3 => "マシ",
        _ => "マシマシ",
    };
    res
}

fn all_spell(i: usize) -> String {
    let mut res = "ゼン".to_string();
    res += match i {
        0 => "ナシ",
        1 => "スクナメ",
        3 => "マシ",
        _ => "マシマシ",
    };
    res
}

fn additional_spell(i: usize, j: usize) -> String {
    let mut res = match i {
        0 => "ヤサイ",
        1 => "ニンニク",
        _ => "アブラ",
    }
    .to_string();
    res += match j {
        0 => "ナシ",
        1 => "スクナメ",
        2 => "フツウ",
        3 => "マシ",
        _ => "マシマシ",
    };
    res
}
