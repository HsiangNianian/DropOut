mod launcher;
mod ui;

fn main() {
    // 初始化UI模块
    ui::init();

    // 启动器模块的逻辑
    launcher::start();
}