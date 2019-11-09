use std::collections::{HashSet, BTreeSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Need one argument, \"hash\" or \"match\".");
        process::exit(1);
    }
    let counted = match &args[1][..] {
        "hash" => Ok(with_hash().unwrap()),
        "match" => Ok(with_match().unwrap()),
        "btree" => Ok(with_btree().unwrap()),
        _ => {
            eprintln!("You can use only \"hash\", \"match\" or \"btree\"");
            Err(())
        }
    };

    match counted {
        Ok(c) => println!("{}", c),
        Err(_) => eprintln!("Something went wrong ☹️ ."),
    }
}

fn with_hash() -> std::io::Result<u32> {
   let hexash: HashSet<u8> = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f',
    ]
    .iter()
    .cloned()
    .collect();
    let mut counter = 0;
    let f = File::open("/usr/share/dict/words")?;

    let reader = BufReader::new(&f);

    for line in reader.lines() {
        let word = line.unwrap();
        let reduction = word.bytes().collect::<HashSet<_>>();

        if word.len() > 2 && reduction.is_subset(&hexash) {
            counter += 1;
        }
    }
    Ok(counter)
}

fn with_btree() -> std::io::Result<u32> {
    let hexas: BTreeSet<u8> = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f',
    ]
    .iter()
    .cloned()
    .collect();
    let mut counter = 0;
    let f = File::open("/usr/share/dict/words")?;

    let reader = BufReader::new(&f);

    for line in reader.lines() {
        let word = line.unwrap();
        let reduction = word.bytes().collect::<BTreeSet<_>>();

        if word.len() > 2 && reduction.is_subset(&hexas) {
            counter += 1;
        }
    }
    Ok(counter)
}

fn with_match() -> std::io::Result<u32> {
    // https://dev.to/timclicks/deadbeef-just-say-no-let-s-learn-to-build-a-small-rust-app-to-find-out-what-words-can-you-spell-with-the-letters-a-f-47em
    // https://github.com/timClicks/hexwords/blob/master/src/main.rs
    let mut counter = 0;
    let f = File::open("/usr/share/dict/words")?;
    let reader = BufReader::new(&f);

    'lines: for line in reader.lines() {
        let word = line.unwrap();
        for byte in word.bytes() {
            match byte {
                b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'a' | b'b' | b'c' | b'd' | b'e'
                | b'f' => continue,
                _ => continue 'lines,
            }
        }

        if word.len() > 2 {
            counter += 1;
        }
    }
    Ok(counter)
}
