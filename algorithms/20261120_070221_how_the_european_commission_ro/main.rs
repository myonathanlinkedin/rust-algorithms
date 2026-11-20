mod core;

use core::{FederationTopology, Message, Room, ServerNode, User};

fn main() {
    // Build a simulated EC Matrix federation
    let mut fed = FederationTopology::new();

    // Add servers across EU regions
    fed.add_server(ServerNode::new(1, 10000, "Brussels"));
    fed.add_server(ServerNode::new(2, 8000, "Luxembourg"));
    fed.add_server(ServerNode::new(3, 6000, "Frankfurt"));
    fed.add_server(ServerNode::new(4, 5000, "Madrid"));

    // Add users (simulating 40k staff rollout)
    for i in 0..40000u32 {
        let server = (i % 4) + 1;
        let active = i % 5 != 0; // 80% active
        fed.add_user(User::new(i, server, active));
    }

    // Add rooms
    fed.add_room(Room::new(1, "EC-General", 40000, true));
    fed.add_room(Room::new(2, "DG-IT", 5000, true));
    fed.add_room(Room::new(3, "Team-Alpha", 50, false));
    fed.add_room(Room::new(4, "Security-Channel", 200, true));

    // Add messages
    fed.add_message(Message::new(1, 1, 1, "Welcome to Matrix", 1000));
    fed.add_message(Message::new(2, 1, 2, "Hello everyone", 1001));
    fed.add_message(Message::new(3, 2, 3, "IT update", 1002));
    fed.add_message(Message::new(4, 1, 4, "Rollout complete", 1003));

    // Assertions
    assert_eq!(fed.servers.len(), 4);
    assert_eq!(fed.users.len(), 40000);
    assert_eq!(fed.active_user_count(), 32000);
    assert_eq!(fed.total_server_capacity(), 29000);
    assert_eq!(fed.rooms.len(), 4);
    assert_eq!(fed.messages.len(), 4);

    // Test users_by_server
    let by_server = fed.users_by_server();
    assert_eq!(by_server.len(), 4);
    assert_eq!(by_server[&1].len(), 10000);
    assert_eq!(by_server[&2].len(), 10000);
    assert_eq!(by_server[&3].len(), 10000);
    assert_eq!(by_server[&4].len(), 10000);

    // Test large_rooms
    let large = fed.large_rooms(1000);
    assert_eq!(large.len(), 2);
    assert_eq!(large[0].name, "EC-General");
    assert_eq!(large[1].name, "DG-IT");

    // Test messages_in_room
    let msgs = fed.messages_in_room(1);
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[0].content, "Welcome to Matrix");
    assert_eq!(msgs[2].content, "Rollout complete");

    // Test health score
    let health = fed.health_score();
    assert!(health > 0.0 && health <= 1.0);

    // Test rollout simulation
    let dist = fed.rollout_simulation(40000, 4);
    assert_eq!(dist.len(), 4);
    assert_eq!(dist.iter().sum::<u32>(), 40000);
    assert_eq!(dist[0], 10000);
    assert_eq!(dist[1], 10000);
    assert_eq!(dist[2], 10000);
    assert_eq!(dist[3], 10000);

    // Test uneven rollout
    let dist2 = fed.rollout_simulation(40001, 4);
    assert_eq!(dist2.iter().sum::<u32>(), 40001);
    assert_eq!(dist2[0], 10001);
    assert_eq!(dist2[1], 10000);

    // Test empty federation
    let empty = FederationTopology::new();
    assert_eq!(empty.active_user_count(), 0);
    assert_eq!(empty.total_server_capacity(), 0);
    assert_eq!(empty.health_score(), 0.0);
    assert_eq!(empty.rollout_simulation(100, 0).len(), 0);

    // Test display
    let display = format!("{}", fed);
    assert!(display.contains("FederationTopology"));
    assert!(display.contains("Servers: 4"));

    println!("All tests passed.");
    println!("{}", fed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_node_creation() {
        let node = ServerNode::new(1, 1000, "Test");
        assert_eq!(node.id, 1);
        assert_eq!(node.capacity, 1000);
        assert_eq!(node.region, "Test");
    }

    #[test]
    fn test_user_creation() {
        let user = User::new(1, 1, true);
        assert_eq!(user.id, 1);
        assert_eq!(user.home_server, 1);
        assert!(user.active);
    }

    #[test]
    fn test_room_creation() {
        let room = Room::new(1, "Test Room", 100, true);
        assert_eq!(room.id, 1);
        assert_eq!(room.name, "Test Room");
        assert_eq!(room.member_count, 100);
        assert!(room.is_encrypted);
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::new(1, 1, 1, "Hello", 1000);
        assert_eq!(msg.id, 1);
        assert_eq!(msg.room_id, 1);
        assert_eq!(msg.sender_id, 1);
        assert_eq!(msg.content, "Hello");
        assert_eq!(msg.timestamp, 1000);
    }

    #[test]
    fn test_federation_empty() {
        let fed = FederationTopology::new();
        assert_eq!(fed.servers.len(), 0);
        assert_eq!(fed.users.len(), 0);
        assert_eq!(fed.rooms.len(), 0);
        assert_eq!(fed.messages.len(), 0);
    }

    #[test]
    fn test_rollout_even_distribution() {
        let fed = FederationTopology::new();
        let dist = fed.rollout_simulation(100, 5);
        assert_eq!(dist, vec![20, 20, 20, 20, 20]);
    }

    #[test]
    fn test_rollout_uneven_distribution() {
        let fed = FederationTopology::new();
        let dist = fed.rollout_simulation(103, 5);
        assert_eq!(dist, vec![21, 21, 21, 20, 20]);
    }

    #[test]
    fn test_rollout_zero_servers() {
        let fed = FederationTopology::new();
        let dist = fed.rollout_simulation(100, 0);
        assert_eq!(dist.len(), 0);
    }

    #[test]
    fn test_rollout_zero_staff() {
        let fed = FederationTopology::new();
        let dist = fed.rollout_simulation(0, 5);
        assert_eq!(dist, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_health_score_empty() {
        let fed = FederationTopology::new();
        assert_eq!(fed.health_score(), 0.0);
    }

    #[test]
    fn test_health_score_low_utilization() {
        let mut fed = FederationTopology::new();
        fed.add_server(ServerNode::new(1, 1000, "Test"));
        for i in 0..100u32 {
            fed.add_user(User::new(i, 1, true));
        }
        let health = fed.health_score();
        assert!(health > 0.8);
    }

    #[test]
    fn test_health_score_high_utilization() {
        let mut fed = FederationTopology::new();
        fed.add_server(ServerNode::new(1, 100, "Test"));
        for i in 0..90u32 {
            fed.add_user(User::new(i, 1, true));
        }
        let health = fed.health_score();
        assert!(health < 0.7);
    }

    #[test]
    fn test_messages_sorted_by_timestamp() {
        let mut fed = FederationTopology::new();
        fed.add_message(Message::new(1, 1, 1, "B", 2000));
        fed.add_message(Message::new(2, 1, 1, "A", 1000));
        fed.add_message(Message::new(3, 1, 1, "C", 3000));
        let msgs = fed.messages_in_room(1);
        assert_eq!(msgs[0].content, "A");
        assert_eq!(msgs[1].content, "B");
        assert_eq!(msgs[2].content, "C");
    }

    #[test]
    fn test_large_rooms_filter() {
        let mut fed = FederationTopology::new();
        fed.add_room(Room::new(1, "Small", 10, false));
        fed.add_room(Room::new(2, "Large", 1000, true));
        fed.add_room(Room::new(3, "Medium", 500, false));
        let large = fed.large_rooms(100);
        assert_eq!(large.len(), 2);
    }
}