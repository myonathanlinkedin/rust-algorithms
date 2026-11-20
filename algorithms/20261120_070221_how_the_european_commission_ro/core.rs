use std::collections::HashMap;
use std::fmt;

/// Represents a Matrix server node in the federation.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ServerNode {
    pub id: u32,
    pub capacity: u32,
    pub region: String,

}

impl ServerNode {
    pub fn new(id: u32, capacity: u32, region: &str) -> Self {
        ServerNode {
            id,
            capacity,
            region: region.to_string(),
        }
    }
}

/// Represents a user account on the Matrix platform.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub home_server: u32,
    pub active: bool,

}

impl User {
    pub fn new(id: u32, home_server: u32, active: bool) -> Self {
        User {
            id,
            home_server,
            active,
        }
    }
}

/// Represents a room in the Matrix protocol.
#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub member_count: u32,
    pub is_encrypted: bool,

}

impl Room {
    pub fn new(id: u32, name: &str, member_count: u32, is_encrypted: bool) -> Self {
        Room {
            id,
            name: name.to_string(),
            member_count,
            is_encrypted,
        }
    }
}

/// Represents a message event in a Matrix room.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: u32,
    pub room_id: u32,
    pub sender_id: u32,
    pub content: String,
    pub timestamp: u64,

}

impl Message {
    pub fn new(id: u32, room_id: u32, sender_id: u32, content: &str, timestamp: u64) -> Self {
        Message {
            id,
            room_id,
            sender_id,
            content: content.to_string(),
            timestamp,
        }
    }
}

/// Federation topology representing the Matrix server network.
#[derive(Debug, Clone, PartialEq)]
pub struct FederationTopology {
    pub servers: Vec<ServerNode>,
    pub users: Vec<User>,
    pub rooms: Vec<Room>,
    pub messages: Vec<Message>,

}

impl FederationTopology {
    pub fn new() -> Self {
        FederationTopology {
            servers: Vec::new(),
            users: Vec::new(),
            rooms: Vec::new(),
            messages: Vec::new(),
        }
    }

    /// Add a server node to the federation.
    pub fn add_server(&mut self, server: ServerNode) {
        self.servers.push(server);
    }

    /// Add a user to the federation.
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    /// Add a room to the federation.
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    /// Add a message to the federation.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Get total number of active users.
    pub fn active_user_count(&self) -> u32 {
        self.users.iter().filter(|u| u.active).count() as u32
    }

    /// Get total capacity of all servers.
    pub fn total_server_capacity(&self) -> u32 {
        self.servers.iter().map(|s| s.capacity).sum()
    }

    /// Get users grouped by home server.
    pub fn users_by_server(&self) -> HashMap<u32, Vec<User>> {
        let mut map: HashMap<u32, Vec<User>> = HashMap::new();
        for user in &self.users {
            map.entry(user.home_server).or_insert_with(Vec::new).push(user.clone());
        }
        map
    }

    /// Get rooms with more than a threshold number of members.
    pub fn large_rooms(&self, threshold: u32) -> Vec<Room> {
        self.rooms
            .iter()
            .filter(|r| r.member_count > threshold)
            .cloned()
            .collect()
    }

    /// Get messages in a specific room, sorted by timestamp.
    pub fn messages_in_room(&self, room_id: u32) -> Vec<Message> {
        let mut msgs: Vec<Message> = self
            .messages
            .iter()
            .filter(|m| m.room_id == room_id)
            .cloned()
            .collect();
        msgs.sort_by_key(|m| m.timestamp);
        msgs
    }

    /// Calculate federation health score based on server utilization.
    pub fn health_score(&self) -> f64 {
        if self.servers.is_empty() {
            return 0.0;
        }
        let total_capacity = self.total_server_capacity() as f64;
        let total_users = self.active_user_count() as f64;
        if total_capacity == 0.0 {
            return 0.0;
        }
        let utilization = total_users / total_capacity;
        // Health is high when utilization is moderate (0.3-0.7)
        if utilization < 0.3 {
            0.8 + utilization
        } else if utilization <= 0.7 {
            1.0 - (utilization - 0.3) * 0.5
        } else {
            0.7 - (utilization - 0.7) * 0.5
        }
    }

    /// Simulate rolling out Matrix to N staff members across M servers.
    /// Returns the distribution of users per server.
    pub fn rollout_simulation(&self, staff_count: u32, server_count: u32) -> Vec<u32> {
        if server_count == 0 {
            return Vec::new();
        }
        let base = staff_count / server_count;
        let remainder = staff_count % server_count;
        let mut distribution = vec![base; server_count as usize];
        for i in 0..remainder as usize {
            distribution[i] += 1;
        }
        distribution
    }
}

impl fmt::Display for FederationTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FederationTopology:")?;
        writeln!(f, "  Servers: {}", self.servers.len())?;
        writeln!(f, "  Users: {}", self.users.len())?;
        writeln!(f, "  Active Users: {}", self.active_user_count())?;
        writeln!(f, "  Rooms: {}", self.rooms.len())?;
        writeln!(f, "  Messages: {}", self.messages.len())?;
        writeln!(f, "  Total Capacity: {}", self.total_server_capacity())?;
        writeln!(f, "  Health Score: {:.4}", self.health_score())?;
        Ok(())
    }
}