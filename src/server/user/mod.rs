pub struct User{
    username: String,
    password: String,
    files: Vec<String>
}

impl User{
    pub fn get_username(&self) -> &str{
        &self.username
    }

    pub fn set_username(&mut self, username: String){
        self.username = username
    }

    pub fn get_password(&self) -> &str{
        &self.password
    }

    pub fn set_password(&mut self, password: String){
        self.password = password
    }

    pub fn get_files(&self) -> &Vec<String>{
        &self.files
    }

    pub fn get_file(&self, index: usize) -> &String{
        &self.files[index]
    }

    pub fn add_file(&mut self, file_name: String){
        self.files.push(file_name)
    }

    pub fn delete_file(&mut self, name: &String) -> Result<(), ()>{

        for (index, file) in self.files.iter().enumerate(){
            if file.eq(name){
                self.files.remove(index);
                return Ok(())
            }
        }
        Err(())
    }
}