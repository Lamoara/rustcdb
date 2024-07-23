#[derive(Debug)]
pub struct Command {
    data: Vec<Vec<u8>>,
}

impl Command {
    // Create a new Command with a command type
    pub fn new(cmd_type: u8) -> Command {
        Command {
            data: vec![vec![cmd_type]],
        }
    }

    // Create a Command from a byte array
    pub fn new_from_b_array(buf: Vec<u8>) -> Command {
        Command {
            data: buf.split(|&b| b == b'|').map(|s| s.to_vec()).collect(),
        }
    }

    // Add an argument to the Command
    pub fn add_arg(&mut self, arg: &str) {
        self.data.push(arg.as_bytes().to_vec());
    }

    // Get the command as a byte vector
    pub fn get_cmd(self) -> Vec<u8> {
        self.data.join(&b'|')
    }

    // Get an argument by index
    pub fn get_arg(&self, index: usize) -> Option<&str> {
        self.data.get(index).and_then(|vec| std::str::from_utf8(vec).ok())
    }
}