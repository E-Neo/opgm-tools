use rand::{Rng, SeedableRng};

const SEED: u64 = 19491001;

pub fn sqlite3_to_sqlite3(
    old_conn: &sqlite::Connection,
    new_conn: &sqlite::Connection,
    num_vlabels: usize,
    num_elabels: usize,
) -> (usize, usize) {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(SEED);
    create_tables(new_conn).unwrap();
    (
        write_vertices(old_conn, new_conn, num_vlabels, &mut rng),
        write_edges(old_conn, new_conn, num_elabels, &mut rng),
    )
}

fn create_tables(conn: &sqlite::Connection) -> sqlite::Result<()> {
    conn.execute("CREATE TABLE vertices (vid INT, vlabel INT)")
        .unwrap();
    conn.execute("CREATE TABLE edges (src INT, dst INT, elabel INT)")
}

fn write_vertices<R: Rng + ?Sized>(
    old_conn: &sqlite::Connection,
    new_conn: &sqlite::Connection,
    num_vlabels: usize,
    rng: &mut R,
) -> usize {
    let mut count = 0;
    let mut old_stat = old_conn.prepare("SELECT * FROM vertices").unwrap();
    new_conn.execute("BEGIN").unwrap();
    let mut stat = new_conn
        .prepare("INSERT INTO vertices VALUES (?, ?)")
        .unwrap();
    while let sqlite::State::Row = old_stat.next().unwrap() {
        let vid: i64 = old_stat.read(0).unwrap();
        stat.bind(1, vid).unwrap();
        stat.bind(2, rng.gen_range(0..num_vlabels) as i64).unwrap();
        stat.next().unwrap();
        stat.reset().unwrap();
        count += 1;
    }
    new_conn.execute("END").unwrap();
    count
}

fn write_edges<R: Rng + ?Sized>(
    old_conn: &sqlite::Connection,
    new_conn: &sqlite::Connection,
    num_elabels: usize,
    rng: &mut R,
) -> usize {
    let mut count = 0;
    let mut old_stat = old_conn.prepare("SELECT * FROM edges").unwrap();
    new_conn.execute("BEGIN").unwrap();
    let mut stat = new_conn
        .prepare("INSERT INTO edges VALUES (?, ?, ?)")
        .unwrap();
    while let sqlite::State::Row = old_stat.next().unwrap() {
        let src: i64 = old_stat.read(0).unwrap();
        let dst: i64 = old_stat.read(1).unwrap();
        stat.bind(1, src).unwrap();
        stat.bind(2, dst).unwrap();
        stat.bind(3, rng.gen_range(0..num_elabels) as i64).unwrap();
        stat.next().unwrap();
        stat.reset().unwrap();
        count += 1;
    }
    new_conn.execute("END").unwrap();
    count
}
