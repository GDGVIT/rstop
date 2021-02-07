use crate::logger::Logger;
use queue::Queue;
use sysinfo::{DiskExt, ProcessorExt, SystemExt};
use termion::event::Key;

#[derive(Clone)]
pub struct App {
    pub name: String,
    pub temps: Vec<Vec<String>>,
    pub disk_usage: Vec<Vec<String>>,
    pub should_quit: bool,
    pub cpu_usage_queue: Queue<(f64, f64)>,
    pub cpu_usage_points: Vec<(f64, f64)>,
    pub max_capacity_queue: usize,
}

impl App {
    pub fn new(name: &str, max_capacity_queue: usize) -> App {
        App {
            name: String::from(name),
            temps: vec![vec![]],
            disk_usage: vec![vec![]],
            should_quit: false,
            cpu_usage_queue: Queue::with_capacity(max_capacity_queue),
            cpu_usage_points: vec![],
            max_capacity_queue,
        }
    }

    pub fn refresh<T: SystemExt>(&mut self, system: &mut T, logger: &mut Logger) {
        //Setting the Temperatures section data
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

        if let Ok(_) = logger.add_log(format!("Temp Entry Data Point: {} - ", self.temps[0][0])) {}

        //Setting the Disk Usage section data
        let disk_usage: Vec<Vec<String>> = system
            .get_disks()
            .iter()
            .map(|x| {
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

        //Setting the cpu_usage section data
        let x = &system.get_processors()[0];
        let l = self.cpu_usage_points.len();

        let mut log: String = String::from("");
        let mut q: Queue<(f64, f64)> = Queue::with_capacity(self.max_capacity_queue);

        if l < self.max_capacity_queue {
            if let Ok(_) = self
                .cpu_usage_queue
                .queue((l as f64, x.get_cpu_usage() as f64))
            {}

            if let Ok(_) = logger.add_log(format!("Entry Data Point: {} - ", x.get_cpu_usage())) {}

            if let Some(ele) = self.cpu_usage_queue.peek() {
                log += &format!("Added: ({}, {}),\t", ele.0, ele.1,);
            }
        } else {
            self.cpu_usage_queue
                .force_queue((0.0, x.get_cpu_usage() as f64));

            let v = self.cpu_usage_queue.vec();

            for (i, ele) in v.iter().enumerate() {
                if let Ok(_) = q.queue((i as f64, ele.1)) {
                    //self.cpu_usage_points
                }
                log += &format!("({}, {}), ", i as f64, ele.1);
            }

            self.cpu_usage_points = self.cpu_usage_queue.vec().to_owned();
            self.cpu_usage_queue = q;

            if let Ok(_) = logger.add_log(log) {}
        }

        if let Ok(_) = logger.add_log("\nIteration Over\n") {}

        //let _a: Vec<()> = system
        //    .get_processors()
        //    .iter()
        //    .map(|x| {
        //    })
        //    .collect();
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
