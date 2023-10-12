#![allow(non_snake_case, unused_macros)]

use itertools::*;
use proconio::input;
use rand::prelude::*;
use svg::node::{
    element::{Group, Line, Rectangle, Style, Title},
    Text,
};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub cs: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.m)?;
        for i in 0..self.n {
            writeln!(f, "{}", self.cs[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize, m: usize,
        cs: [[usize; n]; n]
    }
    Input { n, m, cs }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr>(
    token: Option<&str>,
    lb: T,
    ub: T,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if v < lb || ub < v {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    pub out: Vec<Vec<Vec<usize>>>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut outs = vec![];
    let mut out = vec![];
    for line in f.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut cs = vec![];
        let mut tokens = line.split_whitespace();
        for _ in 0..input.n {
            cs.push(read(tokens.next(), 0, input.m)?);
        }
        if tokens.next().is_some() {
            return Err("illegal output format".to_owned());
        }
        out.push(cs);
        if out.len() == input.n {
            outs.push(out);
            out = vec![];
        }
    }
    if out.len() > 0 {
        return Err("illegal output format".to_owned());
    }
    Ok(Output { out: outs })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let n = 50;
    let m = 100;
    let mut cs = mat![0; n; n];
    let mut list = vec![];
    for i in 0..n {
        for j in 0..n {
            list.push((i, j));
        }
    }
    for i in 0..m {
        let k = rng.gen_range(0..list.len() as i32) as usize;
        let (x, y) = list.swap_remove(k);
        cs[x][y] = i + 1;
    }
    while list.len() > 0 {
        let k = rng.gen_range(0..list.len() as i32) as usize;
        let (x, y) = list.swap_remove(k);
        let d = rng.gen_range(0..4i32) as usize;
        let x2 = x + DIJ[d].0;
        let y2 = y + DIJ[d].1;
        if x2 < n && y2 < n && cs[x2][y2] > 0 {
            cs[x][y] = cs[x2][y2];
        } else {
            list.push((x, y));
        }
    }
    Input { n, m, cs }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    if let Some(out) = out.out.last() {
        let (mut score, err, _) = compute_score_details(input, out);
        if err.len() > 0 {
            score = 0;
        }
        (score, err)
    } else {
        (0, "empty output".to_owned())
    }
}

fn get_adj(cs: &Vec<Vec<usize>>, m: usize) -> Vec<Vec<bool>> {
    let n = cs.len();
    let mut adj = mat![false; m + 1; m + 1];
    for i in 0..n {
        for j in 0..n {
            if cs[i][j] != 0 && (i == 0 || j == 0 || i + 1 == n || j + 1 == n) {
                adj[cs[i][j]][0] = true;
                adj[0][cs[i][j]] = true;
            }
            if i + 1 < n && cs[i][j] != cs[i + 1][j] {
                adj[cs[i][j]][cs[i + 1][j]] = true;
                adj[cs[i + 1][j]][cs[i][j]] = true;
            }
            if j + 1 < n && cs[i][j] != cs[i][j + 1] {
                adj[cs[i][j]][cs[i][j + 1]] = true;
                adj[cs[i][j + 1]][cs[i][j]] = true;
            }
        }
    }
    adj
}

pub fn compute_score_details(input: &Input, out: &Vec<Vec<usize>>) -> (i64, String, ()) {
    let adj0 = get_adj(&input.cs, input.m);
    let adj1 = get_adj(out, input.m);
    for i in 0..=input.m {
        for j in i + 1..=input.m {
            if adj0[i][j] && !adj1[i][j] {
                return (0, format!("Colors {} and {} must be adjacent.", i, j), ());
            } else if !adj0[i][j] && adj1[i][j] {
                return (0, format!("Colors {} and {} must not be adjacent.", i, j), ());
            }
        }
    }
    let mut done = vec![false; input.m + 1];
    let mut visited = mat![false; input.n; input.n];
    for i in 0..input.n {
        for j in 0..input.n {
            if !visited[i][j] {
                if out[i][j] > 0 && !done[out[i][j]].setmax(true) {
                    return (0, format!("Squares of color {} are not connected.", out[i][j]), ());
                }
                let mut stack = vec![(i, j)];
                visited[i][j] = true;
                let mut outer = false;
                while let Some((i, j)) = stack.pop() {
                    for (di, dj) in DIJ {
                        let i2 = i + di;
                        let j2 = j + dj;
                        if i2 >= input.n || j2 >= input.n {
                            outer = true;
                        } else if out[i][j] == out[i2][j2] && visited[i2][j2].setmax(true) {
                            stack.push((i2, j2));
                        }
                    }
                }
                if out[i][j] == 0 && !outer {
                    return (0, format!("Squares of color {} are not connected.", out[i][j]), ());
                }
            }
        }
    }
    let mut score = 1;
    for i in 0..input.n {
        for j in 0..input.n {
            if out[i][j] == 0 {
                score += 1;
            }
        }
    }
    (score, String::new(), ())
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!("#{:02x}{:02x}{:02x}", r.round() as i32, g.round() as i32, b.round() as i32)
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new().add(Text::new(title)))
}

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    if let Some(out) = out.out.last() {
        let (mut score, err, svg) = vis(input, out, !0);
        if err.len() > 0 {
            score = 0;
        }
        (score, err, svg)
    } else {
        (0, String::new(), String::new())
    }
}

pub fn vis(input: &Input, out: &Vec<Vec<usize>>, focus: usize) -> (i64, String, String) {
    let D = 14;
    let W = D * input.n;
    let H = D * input.n;
    let (score, err, _) = compute_score_details(input, &out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    doc = doc.add(rect(0, 0, W, H, "none").set("stroke", "black").set("stroke-width", 2));
    for i in 0..input.n {
        for j in 0..input.n {
            doc = doc.add(group(format!("({}, {}): {}", i, j, out[i][j])).add(rect(
                D * j,
                D * i,
                D,
                D,
                &if out[i][j] == 0 {
                    "white".to_owned()
                } else {
                    color((out[i][j] - 1) as f64 / input.m as f64)
                },
            )));
            if i + 1 < input.n {
                doc = doc.add(
                    Line::new()
                        .set("x1", D * j)
                        .set("y1", D * (i + 1))
                        .set("x2", D * (j + 1))
                        .set("y2", D * (i + 1))
                        .set("stroke", if out[i][j] != out[i + 1][j] { "black" } else { "gray" })
                        .set(
                            "stroke-width",
                            if out[i][j] != out[i + 1][j] {
                                if focus > 0 && (out[i][j] == focus || out[i + 1][j] == focus) {
                                    5
                                } else {
                                    2
                                }
                            } else {
                                1
                            },
                        ),
                );
            }
            if j + 1 < input.n {
                doc = doc.add(
                    Line::new()
                        .set("x1", D * (j + 1))
                        .set("y1", D * i)
                        .set("x2", D * (j + 1))
                        .set("y2", D * (i + 1))
                        .set("stroke", if out[i][j] != out[i][j + 1] { "black" } else { "gray" })
                        .set(
                            "stroke-width",
                            if out[i][j] != out[i][j + 1] {
                                if focus > 0 && (out[i][j] == focus || out[i][j + 1] == focus) {
                                    5
                                } else {
                                    2
                                }
                            } else {
                                1
                            },
                        ),
                );
            }
        }
    }
    (score, err, doc.to_string())
}
