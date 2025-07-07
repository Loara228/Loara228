use std::process::Command;

use chrono::{Days, NaiveDate};

fn main() {
    // fill();
    // return;
    let shrug = 
r#"⬛⬛⬛⬜⬜⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬛⬛⬛
⬜⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜
⬜⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬛⬜⬛⬜⬛⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜
⬜⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬛⬜⬜⬛⬜⬜⬜⬜⬛⬜⬜⬜
⬜⬜⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜⬜
⬜⬜⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜⬛⬛⬜⬜⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬛⬛⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜⬛⬜⬜⬛⬛⬜⬜⬜⬜⬜"#;

    let weeks = 52;
    let art_w = 28;
    let art_x = weeks / 2 - art_w / 2;
    let date_start = NaiveDate::from_ymd_opt(2022, 1, 2)
        .unwrap()
        .checked_add_days(Days::new(art_x * 7))
        .unwrap();

    let mut cur_x = 0;
    let mut cur_y = 0;    
    for c in shrug.chars() {
        match c {
            '⬛' => {
                let cur_d = date_start.checked_add_days(Days::new(cur_x * 7 + cur_y)).unwrap();
                print!("commit: ");
                commit(cur_d);
            },
            '⬜' => (),
            '\n' => {
                cur_y += 1;
                cur_x = 0;
                continue;
            },
            _ => panic!("?")
        }
        cur_x += 1;
    }
}

fn fill() {
    let mut d = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    while d <= NaiveDate::from_ymd_opt(2022, 12, 31).unwrap() {
        commit(d);
        d = d.checked_add_days(Days::new(1)).unwrap();
    }
}

fn commit(dt: NaiveDate) {
    let dt = format!("{dt}T12:00:00+00:00");
    println!("{dt}");
    
    unsafe {
        std::env::set_var("GIT_AUTHOR_DATE".to_owned(), "2022-01-15T12:00:00+00:00".to_owned());
        std::env::set_var("GIT_COMMITTER_DATE".to_owned(), "2022-01-15T12:00:00+00:00".to_owned());
    }
    Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg(format!("-m \"{}\"", dt))
        .arg(format!("--date=\"{}\"", dt))
        .output()
        .unwrap();
}