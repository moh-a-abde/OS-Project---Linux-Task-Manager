mod app;
mod tui;
mod process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}

