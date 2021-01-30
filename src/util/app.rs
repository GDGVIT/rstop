use sysinfo::{DiskExt, SystemExt};
use termion::event::Key;
pub struct App {
    pub name: String,
    pub temps: Vec<Vec<String>>,
    pub disk_usage: Vec<Vec<String>>,
    pub should_quit: bool,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            name: String::from(name),
            temps: vec![vec![]],
            disk_usage: vec![vec![]],
            should_quit: false,
        }
    }

    pub fn refresh<T: SystemExt>(&mut self, system: &mut T) {
        //Setting the Temperatures section
        let temps: Vec<Vec<String>> = system
            .get_components()
            .iter()
            .map(|x| {
                let s: String = format!("{:?}", x);
                let (mut s1, mut s2): (String, String) = (String::from("k"), String::from(""));
                let mut flag = 0;

                for i in s.chars() {
                    if i == ':' {
                        flag = 1;
                        continue;
                    }

                    if flag == 0 {
                        s1.push(i);
                    } else {
                        s2.push(i);
                    }
                }
                vec![s1, s2]
            })
            .collect();
        self.temps = temps;

        //Setting the Disk Usage section
        let disk_usage: Vec<Vec<String>> = system
            .get_disks()
            .iter()
            .map(|x| {
                //let s = format!("{}");

                let name = if let Some(nam) = x.get_name().to_str() {
                    String::from(nam)
                } else {
                    String::from("")
                };

                let mount_point = if let Some(mount_pt) = x.get_mount_point().to_str() {
                    String::from(mount_pt)
                } else {
                    String::from("")
                };

                let mut avb_space;
                let mut mb_flag = 0;
                let mut space = x.get_available_space() / 1000000;
                avb_space = space.to_string();
                if space > 1000 {
                    space = space / 1000;
                    avb_space = space.to_string();
                    mb_flag = 1;
                }

                if mb_flag == 0 {
                    avb_space += "MB";
                } else {
                    avb_space += "GB";
                }

                let mut v: Vec<String> = vec![];
                v.push(name);
                v.push(mount_point);
                v.push(avb_space);

                v
            })
            .collect();
        self.disk_usage = disk_usage;
    }

    pub fn on_key(&mut self, key: Key) {
        match key {
            Key::Char('q') => {
                self.should_quit = true;
            }
            Key::Char('Q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
