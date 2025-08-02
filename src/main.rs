use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: Option<String>,
    #[arg(short, long)]
    entity: Option<String>,
    #[arg(short, long)]
    shell: bool,
    #[arg(short, long)]
    dev: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let auth = tc_emulator::init(args.profile, None).await;
    tc_emulator::emulate(auth, args.entity, args.dev, args.shell).await;
}
