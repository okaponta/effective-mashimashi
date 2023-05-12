use itertools::{iproduct, Itertools};

fn main() {
    println!("| ヤサイ | ニンニク | アブラ | 呪文 | 長さ |");
    println!("|---|---|---|---|---|");
    let mut ans = vec![];

    for (yasai, ninniku, abura) in iproduct!(0..5, 0..5, 0..5) {
        let yna = vec![yasai, ninniku, abura];
        let mut res = (0..3).into_iter().map(|i| spell(i, yna[i], false)).join("");
        for all in 0..5 {
            if all == 3 {
                continue;
            }
            let mut tmp = all_spell(all);
            for i in 0..3 {
                if yna[i] != all {
                    tmp += &spell(i, yna[i], true);
                }
            }
            if tmp.len() < res.len() {
                res = tmp;
            }
        }
        let mut count = res.chars().count();
        if count == 0 {
            res = "ソノママ".to_string();
            count = 4;
        }
        ans.push(format!(
            "|{}|{}|{}|{}|{}|",
            yna[0], yna[1], yna[2], res, count
        ));
    }
    for s in ans {
        println!("{}", s);
    }
}

fn spell(i: usize, j: usize, additional: bool) -> String {
    if !additional && j == 2 {
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
        2 => "フツウ",
        3 => "マシ",
        _ => "マシマシ",
    };
    res
}

fn all_spell(i: usize) -> String {
    let mut res = "ゼン".to_string();
    res += match i {
        0 => "ブナシ",
        1 => "ブスクナメ",
        3 => "マシ",
        _ => "マシマシ",
    };
    res
}
