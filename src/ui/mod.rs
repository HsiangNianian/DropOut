use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

pub fn init() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Minecraft 启动器");
    window.set_default_size(350, 70);

    let button = Button::with_label("开始游戏");
    button.connect_clicked(|_| {
        println!("开始游戏按钮被点击");
    });

    window.add(&button);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}