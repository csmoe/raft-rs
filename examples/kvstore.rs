extern crate raft;

use raft::storage::MemStorage;
use raft::eraftpb::Entry;
use raft::RaftLog;

fn main() {
    let store = MemStorage::new();

    let mut raft_log = RaftLog::new(store, String::from(""));

    let new_entry = |index, term| {
        let mut ent = Entry::new();
        ent.set_index(index);
        ent.set_term(term);
        ent
    };

    let entries = vec![new_entry(1, 1), new_entry(2, 2), new_entry(3, 3)];

    raft_log.append(&entries);

    println!("raft_log: {}", raft_log.to_string());
    println!("entries: {:?}", raft_log.all_entries());
}
