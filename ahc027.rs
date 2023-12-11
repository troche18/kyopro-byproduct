#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
use ac_library::scc;
use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ptr::null;
use std::vec;
use superslice::Ext;

// 参考: https://qiita.com/rhoo/items/2f647e32f6ff2c6ee056
struct Input {
    n: usize,
    nn: usize,
    dirty: Vec<u32>,
    dirty_sum: u32,
    true_sum: u32,
    graph: Vec<Vec<usize>>,
    dc: [char; 4],
    dc2: [char; 4],
    limit: usize,
    div: usize,
    width: usize,
    turn: usize,
    th: u32,
}

impl Input {
    fn input() -> Input {
        let dc = ['L', 'U', 'R', 'D'];
        let dc2 = ['R', 'D', 'L', 'U'];
        let limit = 5;
        let div = 5;
        let width = 200;
        let turn = 5000;
        let th = 50;
        input! {
            n:usize,
            h:[Chars;n-1],
            v:[Chars;n],
            d:[[u32;n];n],
        }
        let mut graph = vec![vec![]; n * n];
        let dirty = d.concat();
        let dirty_sum = dirty.iter().sum();
        let true_sum = dirty_sum;
        for i in 0..n {
            for j in 0..n {
                for (di, dj) in [(1, 0), (0, 1)] {
                    if i + di >= n || j + dj >= n {
                        continue;
                    }
                    if (di, dj) == (1, 0) {
                        if h[i][j] == '1' {
                            continue;
                        }
                        graph[i * n + j].push(i * n + j + n);
                        graph[i * n + j + n].push(i * n + j);
                    } else {
                        if v[i][j] == '1' {
                            continue;
                        }
                        graph[i * n + j].push(i * n + j + 1);
                        graph[i * n + j + 1].push(i * n + j);
                    }
                }
            }
        }
        Input {
            n,
            nn: n * n,
            dirty,
            dirty_sum,
            true_sum,
            graph,
            dc,
            dc2,
            limit,
            div,
            width,
            turn,
            th,
        }
    }
}

#[derive(Clone)]
struct Cand {
    op: u8,
    parent: usize,
    eval_score: u32,
    takahashi: usize,
    raw_score: u32,
    time: u32,
    prev_time: u32,
    no_visit: u32,
    dirty_sum: u32,
}

impl Cand {
    fn to_node(&self) -> Node {
        Node {
            child: !0,
            prev: !0,
            next: !0,
            op: self.op,
            parent: self.parent,
            takahashi: self.takahashi,
            score: self.raw_score,
            time: self.time,
            prev_time: self.prev_time,
            no_visit: self.no_visit,
            dirty_sum: self.dirty_sum,
        }
    }
}

#[derive(Clone, Default)]
struct Node {
    op: u8,
    parent: usize,
    child: usize,
    prev: usize,
    next: usize,
    takahashi: usize,
    score: u32,
    time: u32,
    prev_time: u32,
    no_visit: u32,
    dirty_sum: u32,
}

#[derive(Clone, PartialEq)]
struct State {
    last_visit: Vec<u32>,
}

impl State {
    fn new(input: &Input) -> State {
        let last_visit = vec![0; input.n * input.n];
        State { last_visit }
    }

    fn apply(&mut self, node: &Node) {
        self.last_visit[node.takahashi] = node.time;
    }

    fn revert(&mut self, node: &Node) {
        self.last_visit[node.takahashi] = node.prev_time;
    }
}

struct BeamSearch {
    state: State,
    leaf: Vec<usize>,
    next_leaf: Vec<usize>,
    nodes: Vec<Node>,
    cur_node: usize,
    free: Vec<usize>,
}

impl BeamSearch {
    fn new(state: State, node: Node) -> BeamSearch {
        const MAX_NODES: usize = 500000;
        let mut nodes = vec![Node::default(); MAX_NODES];
        nodes[0] = node;
        let free = (1..MAX_NODES as usize).rev().collect();

        BeamSearch {
            state,
            nodes,
            free,
            leaf: vec![0],
            next_leaf: vec![],
            cur_node: 0,
        }
    }

    fn add_node(&mut self, cand: Cand) {
        let next = self.nodes[cand.parent as usize].child;
        let new = self.free.pop().expect("MAX_NODEが足りないよ") as usize;
        if next != !0 {
            self.nodes[next as usize].prev = new;
        }
        self.nodes[cand.parent as usize].child = new;

        self.next_leaf.push(new);
        self.nodes[new as usize] = Node {
            next,
            ..cand.to_node()
        };
    }

    fn del_node(&mut self, mut idx: usize) {
        loop {
            self.free.push(idx);
            let Node {
                prev, next, parent, ..
            } = self.nodes[idx as usize];
            assert_ne!(parent, !0, "全てのノードを消そうとしています");
            if prev & next == !0 {
                idx = parent;
                continue;
            }

            if prev != !0 {
                self.nodes[prev as usize].next = next;
            } else {
                self.nodes[parent as usize].child = next;
            }
            if next != !0 {
                self.nodes[next as usize].prev = prev;
            }

            break;
        }
    }

    fn no_dfs(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        loop {
            let Node { next, child, .. } = self.nodes[self.cur_node];
            if next == !0 || child == !0 {
                break;
            }
            self.cur_node = child as usize;
            self.state.apply(&self.nodes[self.cur_node]);
        }

        let root = self.cur_node;
        loop {
            let child = self.nodes[self.cur_node].child;
            if child == !0 {
                self.append_cands(input, self.cur_node, cands);
                loop {
                    if self.cur_node == root {
                        return;
                    }
                    let node = &self.nodes[self.cur_node];
                    self.state.revert(&node);
                    if node.next != !0 {
                        self.cur_node = node.next as usize;
                        self.state.apply(&self.nodes[self.cur_node]);
                        break;
                    }
                    self.cur_node = node.parent as usize;
                }
            } else {
                self.cur_node = child as usize;
                self.state.apply(&self.nodes[self.cur_node]);
            }
        }
    }

    fn enum_cands(&mut self, input: &Input, cands: &mut Vec<Cand>) {
        self.no_dfs(input, cands);
    }

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I) {
        self.next_leaf.clear();
        for cand in cands {
            self.add_node(cand);
        }

        for i in 0..self.leaf.len() {
            let n = self.leaf[i];
            if self.nodes[n as usize].child == !0 {
                self.del_node(n);
            }
        }

        std::mem::swap(&mut self.leaf, &mut self.next_leaf);
    }

    fn restore(&self, mut idx: usize) -> Vec<u8> {
        let mut ret = vec![];
        loop {
            let Node { op, parent, .. } = self.nodes[idx as usize];
            if op == !0 {
                break;
            }
            ret.push(op);
            idx = parent;
        }

        ret.reverse();
        ret
    }

    fn append_cands(&self, input: &Input, idx: usize, cands: &mut Vec<Cand>) {
        let node = &self.nodes[idx];
        assert_eq!(node.child, !0);
        let raw_score = node.score;
        let cur = node.takahashi;
        let mut np = 0;

        for op in 0..4 {
            match op {
                0 => np = cur as i32 - 1,
                2 => np = cur as i32 + 1,
                3 => np = cur as i32 + input.n as i32,
                1 => np = cur as i32 - input.n as i32,
                _ => (),
            }

            if np < 0 || np >= (input.n * input.n) as i32 {
                continue;
            }

            let np_u = np as usize;

            if !input.graph[cur].contains(&np_u) {
                continue;
            }

            let mut n_v = node.no_visit;
            let mut score = raw_score;
            let mut d_s = node.dirty_sum;
            score += d_s;
            score -= ((node.time + 1) - self.state.last_visit[np_u]) * input.dirty[np_u];
            if self.state.last_visit[np_u] == 0 && input.dirty[np_u] <= input.th {
                n_v -= 1;
                score -= ((node.time + 1) - self.state.last_visit[np_u]) * input.th;
                d_s -= input.th;
            }

            let cand = Cand {
                op: op as u8,
                parent: idx,
                eval_score: score,
                takahashi: np_u,
                raw_score: score,
                time: node.time + 1,
                prev_time: self.state.last_visit[np_u],
                no_visit: n_v,
                dirty_sum: d_s,
            };
            cands.push(cand);
        }
    }

    fn solve(&mut self, input: &Input) -> Vec<u8> {
        let mut cands: Vec<Cand> = vec![];

        for i in 0..input.turn {
            if i != 0 {
                cands.sort_unstable_by_key(|s| s.eval_score);
                let mut counter = HashMap::new();

                self.update(
                    cands
                        .iter()
                        .filter(|cand| {
                            let count = counter
                                .entry((
                                    cand.takahashi / input.n / input.div,
                                    cand.takahashi % input.n / input.div,
                                ))
                                .or_insert(0);
                            *count += 1;
                            if counter[&(
                                cand.takahashi / input.n / input.div,
                                cand.takahashi % input.n / input.div,
                            )] > input.limit
                            {
                                false
                            } else {
                                true
                            }
                        })
                        .take(input.width)
                        .cloned(),
                );
            }
            cands.clear();
            self.enum_cands(input, &mut cands);
            assert!(!cands.is_empty());
        }

        let best = cands.into_iter().min_by_key(|a| a.raw_score).unwrap();
        let mut ret = self.restore(best.parent);
        ret.push(best.op);

        ret
    }
}

fn main() {
    let mut input = Input::input();
    let zero = to_zero_path(&input);
    let mut final_ans = vec![];
    let mut best_score = usize::MAX;
    let mut sort_dirty = input.dirty.clone();
    sort_dirty.sort();
    for th in [50, 70, 90] {
        input.th = th;
        let bis = sort_dirty.upper_bound(&th) as u32;
        input.dirty_sum = input.true_sum + bis * th;
        let res = solve(&input);
        let mut cur = 0;
        let mut visited = vec![false; input.nn];
        let mut no_visited = input.nn;
        for &op in &res {
            match op {
                0 => cur -= 1,
                2 => cur += 1,
                3 => cur += input.n,
                1 => cur -= input.n,
                _ => (),
            }
            if !visited[cur] {
                no_visited -= 1;
            }
            visited[cur] = true;
        }
        let mut cur = 0;
        let mut que = VecDeque::new();
        let mut time = 0;
        let mut seen_count = 0;
        let mut last_visit = vec![0; input.nn];
        let dict = HashMap::from([
            (input.n as i32, 'D'),
            (input.n as i32 * -1, 'U'),
            (1, 'R'),
            (-1, 'L'),
        ]);
        // eprintln!("{}", no_visited);

        let mut route = vec![];
        for &op in &res {
            if no_visited > 0 {
                match op {
                    0 => cur -= 1,
                    2 => cur += 1,
                    3 => cur += input.n as i32,
                    1 => cur -= input.n as i32,
                    _ => (),
                }
                que.push_back((1, cur, input.dc[op as usize]));
                while !que.is_empty() {
                    let (i, j, k) = que.pop_back().unwrap();
                    time += 1;
                    if last_visit[j as usize] == 0 {
                        seen_count += 1;
                    }
                    last_visit[j as usize] = time;
                    route.push(k);
                    if i == 1 {
                        for nxt in &input.graph[j as usize] {
                            if !visited[*nxt] {
                                no_visited -= 1;
                                visited[*nxt] = true;
                                que.push_back((2, *nxt as i32, dict[&(j - *nxt as i32)]));
                                que.push_back((1, *nxt as i32, dict[&(*nxt as i32 - j)]));
                            }
                        }
                    }
                }
            } else {
                match op {
                    0 => cur -= 1,
                    2 => cur += 1,
                    3 => cur += input.n as i32,
                    1 => cur -= input.n as i32,
                    _ => (),
                }
                time += 1;
                if last_visit[cur as usize] == 0 {
                    seen_count += 1;
                }
                last_visit[cur as usize] = time;
                route.push(input.dc[op as usize]);
                if seen_count == input.nn {
                    let mut cp = route.clone();
                    cp.extend(zero[cur as usize].clone());
                    let sc = calc_route_score(&input, &cp);
                    if sc < best_score {
                        best_score = sc;
                        final_ans = cp.clone();
                    }
                }
            }
        }
    }
    println!("{}", final_ans.iter().join(""));
}

fn solve(input: &Input) -> Vec<u8> {
    let state = State::new(input);
    let node = Node {
        op: !0,
        parent: !0,
        child: !0,
        prev: !0,
        next: !0,
        takahashi: 0,
        score: 0,
        time: 0,
        prev_time: 0,
        no_visit: (input.n * input.n) as u32,
        dirty_sum: input.dirty_sum,
    };
    let mut beam = BeamSearch::new(state, node);
    beam.solve(input)
}

fn to_zero_path(input: &Input) -> Vec<Vec<char>> {
    let mut visited = vec![false; input.nn];
    let mut que = VecDeque::new();
    que.push_back(0);
    visited[0] = true;
    let mut ret = vec![vec![]; input.nn];
    let dict = HashMap::from([
        (input.n as i32, 'D'),
        (input.n as i32 * -1, 'U'),
        (1, 'R'),
        (-1, 'L'),
    ]);

    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        for next in &input.graph[v] {
            if visited[*next] {
                continue;
            }
            let mut cp = ret[v].clone();
            cp.push(dict[&(v as i32 - *next as i32)]);
            ret[*next] = cp;
            visited[*next] = true;
            que.push_back(*next);
        }
    }
    for i in 0..input.nn {
        ret[i].reverse();
    }
    ret
}

fn calc_route_score(input: &Input, route: &Vec<char>) -> usize {
    let mut last_visit = vec![0; input.nn];
    let mut cur = 0;
    let mut total = 0;
    let mut time = 0;
    for &p in route {
        match p {
            'D' => cur += input.n,
            'U' => cur -= input.n,
            'L' => cur -= 1,
            'R' => cur += 1,
            _ => (),
        }
        time += 1;
        total += input.true_sum;
        total -= (time - last_visit[cur]) * input.dirty[cur];
        last_visit[cur] = time;
    }
    let mut ave = 0;
    for &p in route {
        match p {
            'D' => cur += input.n,
            'U' => cur -= input.n,
            'L' => cur -= 1,
            'R' => cur += 1,
            _ => (),
        }
        time += 1;
        total += input.true_sum;
        total -= (time - last_visit[cur]) * input.dirty[cur];
        last_visit[cur] = time;
        ave += total as usize;
    }
    return ave / route.len();
}
