pub struct File{
    name: String,
    connections: Vec<String>
}

impl File{
    pub fn new(file_name: String) -> File{
        File { name: file_name, connections: Vec::new() }
    }

    pub fn is_named(&self, name: &str) -> bool{
        self.name.eq(name)
    }

    pub fn add_connection(&mut self, other_file: &str){
        self.connections.push(other_file.to_string())
    }
    
    pub fn get_connections(&self) -> &Vec<String>{
        &self.connections
    }
}