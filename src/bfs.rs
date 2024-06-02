use std::collections::{HashMap, HashSet, VecDeque};
use crate::shared::Territory;


//BFS Routing
pub fn fastest_routing(start: &str, end: &str, territories: &HashMap<String, Territory>) -> Option<Vec<String>> {
  let mut queue: VecDeque<String> = VecDeque::new();
  let mut parents: HashMap<String, Option<String>> = HashMap::new();
  let mut visited: HashSet<String> = HashSet::new();

  queue.push_back(start.to_owned());
  visited.insert(start.to_owned());
  parents.insert(start.to_owned(), None);

  while let Some(current) = queue.pop_front() {
    if current == end {
      let mut path: Vec<String> = Vec::new();
      let mut node: String = end.to_owned();
      while let Some(parent) = parents.get(&node)?.as_deref() {
        path.push(node.clone());
        node = parent.to_owned();
      }
      path.push(start.to_owned());
      path.reverse();
      return Some(path);
    }

    if let Some(territory) = territories.get(&current) {
      for neighbor in &territory.neighbors {
        if visited.contains(neighbor) {
          continue;
        }

        queue.push_back(neighbor.to_owned());
        visited.insert(neighbor.to_owned());
        parents.insert(neighbor.to_owned(), Some(current.to_owned()));
      }
    }
  }

  None
}





