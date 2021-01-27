use sysinfo::SystemExt;
pub struct App {
    pub name: String,
    pub temps: Vec<String>,
    pub disk_usage: Vec<String>,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            name: String::from(name),
            temps: vec![],
            disk_usage: vec![],
        }
    }

    pub fn refresh<T: SystemExt>(&mut self, system: &mut T) {
        let temps: Vec<String> = system
            .get_components()
            .iter()
            .map(|x| {
                let s: String = format!("{:?}", x);
                println!("{}", s);
                for i in s.chars() {
                    println!("{}", i);
                }
                s
            })
            .collect();
        self.temps = temps;
        println!("{:#?}", self.temps);
    }
}
