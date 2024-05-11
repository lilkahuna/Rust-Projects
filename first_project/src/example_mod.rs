pub mod hosting {
    pub struct Registration {
        pub group_size: u8,
        pub tab: u32,
        pub server: String
    }
    
    // Associated Functions
    impl Registration {
        pub fn create_reg(group_size: u8, tab: u32, server: String) -> Self {
            Self {
                group_size: group_size,
                tab: tab,
                server: server
            }
        }
    }

    // Methods for instances
    impl Registration {
        pub fn add_to_tab(&mut self, amount: u32) {
            self.tab += amount
        }
    }
}