use crate::types::{BipartiteGraph, MatchingResult};
use std::collections::VecDeque;

/// Computes a maximum cardinality matching in a bipartite graph using the Hopcroft‑Karp algorithm.
pub fn hopcroft_karp(graph: &BipartiteGraph) -> MatchingResult {
    let n_left = graph.n_left;
    let n_right = graph.n_right;

    let mut pair_u: Vec<Option<usize>> = vec![None; n_left];
    let mut pair_v: Vec<Option<usize>> = vec![None; n_right];
    let mut dist: Vec<usize> = vec![0; n_left];
    let mut matching = 0usize;

    while bfs(graph, &pair_u, &pair_v, &mut dist) {
        for u in 0..n_left {
            if pair_u[u].is_none() && dfs(u, graph, &mut pair_u, &mut pair_v, &dist) {
                matching += 1;
            }
        }
    }

    MatchingResult {
        matching_size: matching,
        pair_u,
        pair_v,
    }
}

// Breadth‑first search builds distance layers from all free left vertices.
// Returns true if there exists at least one free right vertex reachable.
pub fn bfs(
    graph: &BipartiteGraph,
    pair_u: &Vec<Option<usize>>,
    pair_v: &Vec<Option<usize>>,
    dist: &mut Vec<usize>,
) -> bool {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let inf = usize::MAX;

    for u in 0..graph.n_left {
        if pair_u[u].is_none() {
            dist[u] = 0;
            queue.push_back(u);
        } else {
            dist[u] = inf;
        }
    }

    let mut found_free_right = false;

    while let Some(u) = queue.pop_front() {
        for &v in &graph.adj[u] {
            if let Some(pu) = pair_v[v] {
                // v is matched; consider the next left vertex in the alternating path
                if dist[pu] == inf {
                    dist[pu] = dist[u] + 1;
                    queue.push_back(pu);
                }
            } else {
                // v is free; we have reached the right side of an augmenting path
                found_free_right = true;
            }
        }
    }

    found_free_right
}

// Depth‑first search attempts to find an augmenting path starting from left vertex `u`.
// Returns true if such a path is found and the matching is updated.
pub fn dfs(
    u: usize,
    graph: &BipartiteGraph,
    pair_u: &mut Vec<Option<usize>>,
    pair_v: &mut Vec<Option<usize>>,
    dist: &Vec<usize>,
) -> bool {
    for &v in &graph.adj[u] {
        if let Some(pu) = pair_v[v] {
            // v is matched; we can continue only if the next left vertex is on the next layer
            if dist[pu] == dist[u] + 1 && dfs(pu, graph, pair_u, pair_v, dist) {
                pair_u[u] = Some(v);
                pair_v[v] = Some(u);
                return true;
            }
        } else {
            // v is free – we can augment directly
            pair_u[u] = Some(v);
            pair_v[v] = Some(u);
            return true;
        }
    }
    false
}