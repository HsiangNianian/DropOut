pub mod config;
pub mod launcher;

pub use config::Config;
pub use launcher::Launcher;

pub fn start() {
    // 启动器的代码
    println!("启动器启动中...");

    // 创建配置
    let config = Config::new("玩家", (1920, 1080));

    // 创建启动器
    let launcher = Launcher::new(config);

    // 启动游戏
    launcher.launch();
}