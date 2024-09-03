pub fn cheapest_routing(world: &World, start: &str, goal: &str) -> Option<Vec<String>> {
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    let mut came_from: HashMap<String, String> = HashMap::new();
    let mut cost_so_far: HashMap<String, Resources> = HashMap::new();

    let start_state: State = State {
        cost: Resources::default(),
        node: start.to_string(),
        estimated_total_cost: Resources::default(), // Heuristic cost is zero for simplicity
    };

    open_set.push(start_state);
    cost_so_far.insert(start.to_string(), Resources::default());

    while let Some(current_state) = open_set.pop() {
        let current_node = current_state.node.clone();

        if current_node == goal {
            let mut path: Vec<String> = Vec::new();
            let mut current: String = goal.to_string();
            while let Some(prev) = came_from.get(&current) {
                path.push(current.clone());
                current = prev.clone();
            }
            path.push(start.to_string());
            path.reverse();
            return Some(path);
        }

        for neighbor in world.get_neighbors(&current_node) {
            let neighbor_territory: &Territory = world.territories.get(&neighbor).unwrap();
            let current_cost: Resources = cost_so_far.get(&current_node).unwrap().clone();
            let new_cost: Resources = current_cost + neighbor_territory.total_cost();

            if !cost_so_far.contains_key(&neighbor)
                || new_cost < *cost_so_far.get(&neighbor).unwrap()
            {
                cost_so_far.insert(neighbor.clone(), new_cost);
                let estimated_total_cost: Resources = new_cost; // For simplicity, heuristic cost is zero

                let neighbor_state = State {
                    cost: new_cost,
                    node: neighbor.clone(),
                    estimated_total_cost,
                };

                open_set.push(neighbor_state);
                came_from.insert(neighbor.clone(), current_node.clone());
            }
        }
    }

    None
}
