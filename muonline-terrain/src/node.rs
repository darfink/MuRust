#![allow(non_upper_case_globals)]
bitflags! {
    /// The attributes of a node.
    pub struct Node: u8 {
        /// Disabled offensive attacks.
        const SafeZone = (1 << 0);
        /// Occupied by an entity.
        const Occupied = (1 << 1);
        /// Wall.
        const Wall     = (1 << 2);
        /// Inaccessible.
        const Void     = (1 << 3);
    }
}

bitflags_serialize!(Node, u8);

impl Node {
    /// Returns whether this is an accessible node or not.
    pub fn is_accessible(&self) -> bool {
        !self.intersects(Node::Wall | Node::Void)
    }

    /// Returns whether this node is free or not.
    pub fn is_free(&self) -> bool {
        !self.intersects(Node::Occupied | Node::Wall | Node::Void)
    }
}
