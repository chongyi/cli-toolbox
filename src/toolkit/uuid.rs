use chrono::Utc;
use clap::{Args, Subcommand, ArgEnum};
use tabled::{Tabled, TableIteratorExt};
use uuid::{Uuid, v1::{Context, Timestamp}};

lazy_static::lazy_static! {
    static ref CLOCK_SEQUENCE: Context = {
        Context::new(0)
    };

    static ref MAC_ADDRESS: [u8; 6] = {
        mac_address::get_mac_address()
            .expect("获取 MAC 地址信息失败")
            .expect("无法获取到有效的 MAC 地址")
            .bytes()
    };
}

#[derive(Args)]
pub struct UuidToolkit {
    #[clap(subcommand)]
    commands: UuidToolkitSubCommand
}

#[derive(Subcommand)]
pub enum UuidToolkitSubCommand {
    GenerateUuid(GenerateUuidArgs)
}

#[derive(Args)]
pub struct GenerateUuidArgs {
    #[clap(short, long, default_value = "1")]
    count: usize,
    #[clap(arg_enum)]
    version: UuidVersion,
}

#[derive(ArgEnum, Clone, Copy)]
pub enum UuidVersion {
    V1,
    V4,
}

impl UuidToolkit {
    pub fn run(self) {
        match self.commands {
            UuidToolkitSubCommand::GenerateUuid(args) => generate_uuid(args.version, args.count),
        }
    }
}

fn generate_uuid(version: UuidVersion, count: usize) {
        let mut buffer = Uuid::encode_buffer();

    #[derive(Tabled)]
    struct Generated {
        #[tabled(rename = "Numerical")]
        number: u128,
        #[tabled(rename = "Hyphenated")]
        hyphenated: String,
        #[tabled(rename = "Simple")]
        simple: String,
    }

    let mut table_data = vec![];

    for _ in 0..count {
        let uuid = match version {
            UuidVersion::V1 => generate_v1_uuid(),
            UuidVersion::V4 => generate_v4_uuid(),
        };

        table_data.push(Generated {
            number: uuid.as_u128(),
            hyphenated: uuid.as_hyphenated().encode_lower(&mut buffer).to_string(),
            simple: uuid.as_simple().encode_lower(&mut buffer).to_string(),
        })
    }

    println!("{}", table_data.table());
}

/// 生成 UUID v1
fn generate_v1_uuid() -> Uuid {
    let now = Utc::now();
    let ctx: &Context = &CLOCK_SEQUENCE;

    Uuid::new_v1(
        Timestamp::from_unix(
            ctx,
            now.timestamp() as u64,
            now.timestamp_subsec_nanos() as u32,
        ),
        &MAC_ADDRESS,
    )
}

/// 生成 UUID v4
fn generate_v4_uuid() -> Uuid {
    Uuid::new_v4()
}