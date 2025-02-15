use clap::{ArgAction, Parser};
use msicrab::utils::common::{get_installed_msis, is_elevated, Win32Product};
use msicrab::utils::msi::{repair_msi, start_procmon};
use msicrab::utils::output::{copy_msi_file, display_products, write_csv_output};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short, action=ArgAction::SetTrue)]
    filter: bool,
    #[clap(long, short, action=ArgAction::SetTrue)]
    csv: bool,
    #[clap(long, short, action=ArgAction::SetTrue)]
    extract: bool,
    #[clap(long, short, action=ArgAction::SetTrue)]
    repair: bool,
    #[clap(long, short, action=ArgAction::SetTrue, requires="repair")]
    procmon: bool,
}

static OUTPUT_DIR: &str = "output";

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if args.procmon && !is_elevated()? {
        println!("Running with Procmon logging enabled requires administrative rights. Aborting.");
        return Ok(());
    }

    println!("Getting a list of installed MSIs. This might take a while.");
    let products: Vec<Win32Product>;
    if args.filter {
        let default_vendors = vec![
            "Microsoft Corporation".to_string(),
            "Python Software Foundation".to_string(),
            "Microsoft Corporations".to_string(),
            "Microsoft".to_string(),
        ];
        products = get_installed_msis(Some(default_vendors))?;
    } else {
        products = get_installed_msis(None)?;
    }

    display_products(&products);

    if args.csv {
        write_csv_output(&products, OUTPUT_DIR)?;
        println!("MSIs have been written to {}/list.csv", OUTPUT_DIR);
    }

    if args.extract {
        for product in &products {
            copy_msi_file(product, OUTPUT_DIR)?;
        }
        println!("MSIs have been copied to to {}", OUTPUT_DIR);
    }

    if args.repair {
        println!("Repairing MSIs. Check the UI for possible Privilege Escalations.\nReference: https://badoption.eu/blog/2023/10/03/MSIFortune.html");
        if args.procmon {
            println!("Starting Procmon with msiexec filter.");
            start_procmon()?;
            println!("Press Enter when you have configured your procmon filter.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }
        for product in &products {
            println!("Press Enter when you are ready for the repair process.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            repair_msi(product)?;
        }
    }
    println!("Nothing left to do!");

    Ok(())
}
