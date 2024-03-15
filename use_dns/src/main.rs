use std::env;

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::rr::RecordType,
    Resolver,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a name to query!");
        std::process::exit(1);
    }

    let query = format!("{}.", args[1]);
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(query.as_str()).unwrap();
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    let resolver = Resolver::from_system_conf().unwrap();

    let response = resolver.lookup_ip(query.as_str()).unwrap();
    for ans in response.iter() {
        println!("{:?}", ans);
    }

    let ns: trust_dns_resolver::lookup::Lookup = resolver.lookup(query.as_str(), RecordType::NS).unwrap();
    for ans in ns.iter() {
        println!("{:?}", ans);
    }
}
