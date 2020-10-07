fn parse_aoj_0002(input: String) -> Vec<(i32, i32)> {
    input
        .lines()
        .take_while(|x| x.len() > 0)
        .map(|x| {
            let v: Vec<_> = x.split(" ").map(|x| x.parse().unwrap()).collect();
            (v[0], v[1])
        })
        .collect()
}

pub fn aoj_0002(input: String) -> String {
    let mut output = "".to_string();
    let ab = parse_aoj_0002(input);
    for (a, b) in ab {
        let res = (a + b).to_string().len().to_string();
        output.push_str(&format!("{}\n", res));
    }
    output
}
