use std::{io, fs, fs::File};
use std::path::Path;
use std::io::{Write, BufReader, Read};


trait MenuAction {
    fn get_title(&self) -> String;
    fn invoke(&self);
}

struct MainMenu {
    menu: Vec<Box<dyn MenuAction>>,

}

impl MainMenu {
    fn print_menu(&self) {
        for (index, menu) in self.menu.iter().enumerate() {
            println!("{}) {}", index + 1, menu.get_title());
        }
    }

    fn run(&self, option: usize) {
        if option <= self.menu.len() && option > 0 {
            self.menu.get(option - 1).expect("Выход за приделы вектора").invoke();
        } else {
            println!("Такого пункта нет в меню {}", option)
        }
    }
}

struct SubMenu {
    title: String,
    action: fn(),
}

impl MenuAction for SubMenu {
    fn get_title(&self) -> String {
        String::from(&self.title)
    }

    fn invoke(&self) {
        (&self.action)();
    }
}


fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

fn main() {
    let first_option = Box::new(SubMenu {
        title: "Указать пути до папки  на пример d:\\folder".to_string(),
        action:
        || write_in_file("core/path.cfg", "Введите путь: "),
    });
    let second_option = Box::new(SubMenu {
        title: "Создать ссылку".to_string(),
        action:
        || create_project(),
    });
    let all_menus: Vec<Box<dyn MenuAction>> = vec![first_option, second_option];
    let main_menu = MainMenu { menu: all_menus };

    loop {
        main_menu.print_menu();
        match get_input().parse::<usize>() {
            Err(_) => println!("Вы должны ввести число !!!"),
            Ok(option) => main_menu.run(option)
        }
    }

}

fn write_in_file(dirs: &str, prompt: &str) {
    let mut vec_dir: Vec<&str> = dirs.split("/").collect();
    vec_dir.pop().unwrap();

    fs::create_dir_all(vec_dir.join("/")).unwrap();
    let path = Path::new(dirs);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    loop {
        println!("{}", prompt);
        let input = get_input();
        if Path::new(&input).is_dir() {
            match file.write_all(input.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => break println!("путь установлен"),
            }
        } else {
            println!("Такого пути не существует: {}", input);
        }
    }
}

fn create_project() {
    match fs::File::open("core/path.cfg") {
        Err(_) => println!("Нужно объязательно создать путь!!"),
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            loop {
                println!("Введите путь где создать проект пример: D:\\my_project ");
                let project_path = get_input();

                let mut full_project_path: Vec<&str> = project_path.split("\\").collect();
                let last_folder = full_project_path.pop().unwrap();
                // let command_project_link = format!("mklink /D {} {}\\{}", project_path, contents, last_folder);
                fs::create_dir_all(format!("{}/{}",contents, last_folder)).unwrap();
                match std::os::windows::fs::symlink_dir(format!("{}/{}",contents,last_folder).as_str(), format!("{}",project_path).as_str()){
                    Err(e) => println!("ОШИБКА ПОПРОБУЙТЕ ЗАПУСТИТЬ ПРОГРАММУ ОТ АДМИНИСТРАТОРА {}", e),
                    Ok(_) => break println!("ПОЗДРАВЛЯЮ ПРОЕКТ СОЗДАН")
                }
            }
        }
    };
}