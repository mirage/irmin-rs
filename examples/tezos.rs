use irmin::*;

fn list_path<T: Contents>(store: &Store<T>, path: Path) -> Result<(), Error> {
    for k in store.list(&path)? {
        let p = path.append_path(&k)?;

        // If the store has contents at `p` then print the path
        if store.mem(&p) {
            println!("{}", p.to_string()?);
        } else {
            list_path(store, p)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        println!("usage: {} /path/to/tezos/context <commit_hash>", &args[0]);
        return Ok(());
    }

    // Configure an in-memory store with `Json` contents
    let mut config = Config::<IrminString>::tezos()?;
    assert!(config.set_root(&args[1]));

    // Initialize the repo
    let repo = Repo::new(config)?;

    // Resolve commit
    let hash = Hash::of_string(&repo, &args[2])?;
    let commit = Commit::of_hash(&repo, &hash).expect("Commit not found");

    // Open the store
    let store = Store::of_commit(&repo, &commit)?;

    // List contract paths
    let path = repo.path(&["data", "contracts"])?;
    list_path(&store, path)?;

    Ok(())
}
