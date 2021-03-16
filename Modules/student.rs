#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[derive(Debug)]
pub struct Student {
    name: String,
    age: u8,
    dept: String,
    cgpa: f32,
    gender: String,
}

impl Student {
    pub fn new(name: String, age: u8, dept: String, cgpa: f32, gender: String) -> Self {
        Student {
            name: name,
            age: age,
            dept: dept,
            cgpa: cgpa,
            gender: gender,
        }
    }
    // Setters
    pub fn set_name(&mut self, name: String) {
        (*self).name = name;
    }
    pub fn set_age(&mut self, age: u8) {
        (*self).age = age;
    }
    pub fn set_dept(&mut self, dept: String) {
        (*self).dept = dept;
    }
    pub fn set_cgpa(&mut self, cgpa: f32) {
        (*self).cgpa = cgpa;
    }
    pub fn set_gender(&mut self, gender: String) {
        (*self).gender = gender;
    }
    // Getters
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_age(&self) -> &u8 {
        &self.age
    }
    pub fn get_dept(&self) -> &String {
        &self.dept
    }
    pub fn get_cgpa(&self) -> &f32 {
        &self.cgpa
    }
    pub fn get_gender(&self) -> &String {
        &self.gender
    }
}

pub fn display_student(student: &Student) {
    println!("{:#?}", student);
}
