use proconio::input;

fn main() {
    input! {
        scores:[i32;5],
    }

    let mut bit = 0;
    let mut result: Vec<(String, i32)> = Vec::new();
    let two = vec![16, 8, 4, 2, 1];
    let spell = vec!["A", "B", "C", "D", "E"];

    for _i in 0..31 {
        let mut name: String = "".to_string();
        let mut score = 0;
        bit += 1;
        let mut buf = bit;
        for j in 0..two.len() {
            if buf >= two[j] {
                name = name + spell[j];
                score += scores[j];
                buf -= two[j];
            }
        }
        result.push((name, score));
    }

    result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    for scr in result.iter() {
        println!("{}", scr.0);
    }
}
