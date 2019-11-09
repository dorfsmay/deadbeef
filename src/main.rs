use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let funcs: &[&dyn Fn(&mut _) -> (&str, u32)] = &[&with_match, &with_hash, &with_btree];

    let f = &File::open("/usr/share/dict/words")?;
    let reader = &mut BufReader::new(f);
    let _load_in_os_fs_cache: Vec<String> = reader.lines().map(|e| e.unwrap()).collect();
    reader.seek(SeekFrom::Start(0))?;

    for algo in funcs.iter() {
        let t0 = Instant::now();
        let (desc, counted) = algo(reader);
        println!("{:?} {} {}", t0.elapsed(), desc, counted);
        reader.seek(SeekFrom::Start(0))?;
    }
    Ok(())
}

fn with_hash<R: BufRead>(reader: &mut R) -> (&str, u32) {
    let description = "one HashSet per word";
    let hexash: HashSet<u8> = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f',
    ]
    .iter()
    .cloned()
    .collect();
    let mut counter = 0;

    for line in reader.lines() {
        let word = line.unwrap();
        let reduction = word.bytes().collect::<HashSet<_>>();

        if word.len() > 2 && reduction.is_subset(&hexash) {
            counter += 1;
        }
    }
    (description, counter)
}

fn with_btree<R: BufRead>(reader: &mut R) -> (&str, u32) {
    let description = "one btree per word";
    let hexas: BTreeSet<u8> = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f',
    ]
    .iter()
    .cloned()
    .collect();
    let mut counter = 0;

    for line in reader.lines() {
        let word = line.unwrap();
        let reduction = word.bytes().collect::<BTreeSet<_>>();

        if word.len() > 2 && reduction.is_subset(&hexas) {
            counter += 1;
        }
    }
    (description, counter)
}

fn with_match<R: BufRead>(reader: &mut R) -> (&str, u32) {
    // https://dev.to/timclicks/deadbeef-just-say-no-let-s-learn-to-build-a-small-rust-app-to-find-out-what-words-can-you-spell-with-the-letters-a-f-47em
    // https://github.com/timClicks/hexwords/blob/master/src/main.rs
    let description = "match trick";
    let mut counter = 0;
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
    (description, counter)
}
