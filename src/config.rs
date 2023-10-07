use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Progress {
    pub level: u8
}

impl Progress {
    pub fn up(&mut self) {
        if self.level + 1 > 10 {
            println!("You have achieved the highest level already!")
        } else {
            self.level += 1;
        }
    }

    pub fn down(&mut self) {
        if self.level - 1 < 1 {
            println!("I understand that but you cannot level down anymore!")
        } else {
            self.level -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.level = 1;
    }
}

impl Default for Progress {
    fn default() -> Progress {
        Progress { level: 10 }
    }
}
