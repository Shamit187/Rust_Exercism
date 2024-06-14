pub fn verse(n: u32) -> String {
    if n == 0 as u32{
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    let end_text = if n > 1 {(n - 1).to_string()} else {String::from("no more")};
    let bottle_initial = if n == 1 {String::from("bottle")} else {String::from("bottles")};
    let bottle_last = if n - 1 == 1 {String::from("bottle")} else {String::from("bottles")};
    format!("{n} {bottle_initial} of beer on the wall, {n} {bottle_initial} of beer.\nTake it down and pass it around, {end_text} {bottle_last} of beer on the wall.\n")
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start+1)
        .rev()
        .map(|beer|{
            verse(beer) + if beer == end {""} else {"\n"}
        })
        .collect()
}
