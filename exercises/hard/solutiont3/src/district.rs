use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
struct UnionFind {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn find(&mut self, node: &str) -> String {
        if !self.parent.contains_key(node) {
            self.parent.insert(node.to_string(), node.to_string());
            self.rank.insert(node.to_string(), 1);
            return node.to_string();
        }
        let mut path = Vec::new();
        let mut current = node.to_string();
        while self.parent[&current] != current {
            path.push(current.clone());
            current = self.parent[&current].clone();
        }
        for p in path {
            self.parent.insert(p.clone(), current.clone());
        }
        current
    }

    fn union(&mut self, a: &str, b: &str) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        let rank_a = *self.rank.get(&root_a).unwrap();
        let rank_b = *self.rank.get(&root_b).unwrap();
        if rank_a > rank_b {
            self.parent.insert(root_b.clone(), root_a.clone());
            self.rank.insert(root_a.clone(), rank_a + rank_b);
        } else {
            self.parent.insert(root_a.clone(), root_b.clone());
            self.rank.insert(root_b.clone(), rank_a + rank_b);
        }
    }

    fn get_roots(&mut self) -> HashSet<String> {
        let mut roots = HashSet::new();
        let keys: Vec<String> = self.parent.keys().cloned().collect();
        for key in keys {
            let root = self.find(&key);
            roots.insert(root);
        }
        roots
    }
}

fn compact_json(s: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape = false;
    for c in s.chars() {
        if c == '"' && !escape {
            in_string = !in_string;
        }
        if c == '\\' && !escape {
            escape = true;
        } else {
            escape = false;
        }
        if in_string {
            result.push(c);
        } else if !c.is_whitespace() {
            result.push(c);
        }
    }
    result
}

fn parse_string(s: &[char], pos: &mut usize) -> String {
    let mut result = String::new();
    *pos += 1;
    while *pos < s.len() {
        match s[*pos] {
            '\\' => {
                *pos += 1;
                result.push(s[*pos]);
            }
            '"' => {
                *pos += 1;
                break;
            }
            c => result.push(c),
        }
        *pos += 1;
    }
    result
}

fn parse_array(s: &[char], pos: &mut usize) -> Vec<String> {
    let mut arr = Vec::new();
    *pos += 1;
    while *pos < s.len() {
        match s[*pos] {
            '"' => arr.push(parse_string(s, pos)),
            ']' => {
                *pos += 1;
                break;
            }
            _ => *pos += 1,
        }
    }
    arr
}

fn parse_city_map(s: &[char], pos: &mut usize) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    *pos += 1;
    while *pos < s.len() {
        if s[*pos] == '"' {
            let key = parse_string(s, pos);
            *pos += 1;
            let val = parse_array(s, pos);
            map.entry(key)
                .and_modify(|v: &mut Vec<String>| v.extend(val.clone()))
                .or_insert(val);
        } else if s[*pos] == '}' {
            *pos += 1;
            break;
        } else {
            *pos += 1;
        }
    }
    map
}

fn parse_top_level(s: &[char], pos: &mut usize) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut batches = HashMap::new();
    *pos += 1;
    while *pos < s.len() {
        if s[*pos] == '"' {
            let batch_key = parse_string(s, pos);
            *pos += 1;
            let city_map = parse_city_map(s, pos);
            batches.insert(batch_key, city_map);
        } else if s[*pos] == '}' {
            *pos += 1;
            break;
        } else {
            *pos += 1;
        }
    }
    batches
}

pub fn count_provinces() -> String {
    let mut file = File::open("district.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let compact = compact_json(&contents);
    let chars: Vec<char> = compact.chars().collect();
    let mut pos = 0;
    let batches = parse_top_level(&chars, &mut pos);

    let mut batch_keys: Vec<String> = batches.keys().cloned().collect();
    batch_keys.sort_by_key(|k| k.parse::<usize>().unwrap());

    let mut results = Vec::new();
    for key in batch_keys {
        let city_map = &batches[&key];
        let mut uf = UnionFind::new();
        for (city, links) in city_map {
            uf.find(city); // 确保城市自身加入集合
            for linked_city in links {
                uf.union(city, linked_city);
            }
        }
        results.push(uf.get_roots().len().to_string());
    }
    results.join(",")
}