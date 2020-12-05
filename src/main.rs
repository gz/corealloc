use clap::{crate_version, value_t, App, Arg};

mod topology;
use topology::*;

fn main() {
    let _r = env_logger::try_init();
    let args = std::env::args();
    let matches = App::new("Core allocation")
        .version(crate_version!())
        .author("Gerd Zellweger <mail@gerdzellweger.com>")
        .about("Figures out which cores IDs to pin my application on.")
        .arg(
            Arg::with_name("cores")
                .short("c")
                .long("cores")
                .help("How many cores do you want?")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tm")
                .long("thread-mapping")
                .short("t")
                .takes_value(true)
                .possible_values(&["sequential", "interleave", "roundrobin"])
                .help("Do you want to interleave over sockets or allocate sequential"),
        )
        .arg(
            Arg::with_name("noht")
                .long("no-ht")
                .short("n")
                .takes_value(false)
                .help("Avoid using hyper-threads"),
        )
        .get_matches_from(args);

    let cores = value_t!(matches, "cores", usize).unwrap_or_else(|e| e.exit());
    let no_ht = matches.is_present("noht");
    let thread_mapping = match matches.value_of("tm").unwrap_or("interleave") {
        "interleave" => ThreadMapping::Interleave,
        "sequential" => ThreadMapping::Sequential,
        "roundrobin" => ThreadMapping::RoundRobin,
        _ => unreachable!(),
    };
    let mt = MachineTopology::new();

    let cpus = mt.allocate(thread_mapping, cores, !no_ht);
    if cores > cpus.len() {
        eprintln!("Don't overprovision cores");
        std::process::exit(1);
    }

    for cpu in cpus {
        print!("{} ", cpu.cpu);
    }
    println!("");
}
