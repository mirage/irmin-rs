use irmin::*;

fn main() -> Result<(), Error> {
    // Configure an in-memory store with `Json` contents
    let config = Config::<Json>::mem(None)?;

    // Initialize the repo
    let repo = Repo::new(config)?;

    // Open the main branch
    let mut store = Store::new(&repo)?;

    // Create the path to store values at
    let path = repo.path(&["foo", "bar"])?;

    // Store a value in the main branch
    let a = json!({
        "x": 1,
        "y": 2,
        "z": 3,
    });
    store.set(
        &path,
        a.as_object().unwrap(),
        Info::new(&repo, "example", "initial commit")?,
    )?;
    let head = store.head().unwrap();

    // Crate `branch1` from the latest commit on `main`
    let mut branch1 = Store::of_branch(&repo, "branch1")?;
    branch1.set_head(&head);

    // Crate `branch2` from the latest commit on `main`
    let mut branch2 = Store::of_branch(&repo, "branch2")?;
    branch2.set_head(&head);

    // Set `x` to 0 and store in `branch1`
    let b = json!({
        "x": 0,
        "y": 2,
        "z": 3,
    });
    branch1.set(
        &path,
        b.as_object().unwrap(),
        Info::new(&repo, "example", "initial commit")?,
    )?;

    // Set `y` to 0 and store in `branch2`
    let c = json!({
        "x": 1,
        "y": 0,
        "z": 3,
    });
    branch2.set(
        &path,
        c.as_object().unwrap(),
        Info::new(&repo, "example", "initial commit")?,
    )?;

    // Merge `branch1` into `main`
    assert!(store.merge(&branch1, repo.info("example", "merge branch1")?));

    // Merge `branch2` into `main`
    assert!(store.merge(&branch2, repo.info("example", "merge branch2")?));

    // Check that the contents have been merged correctly
    let v = store.find(&path)?.unwrap();
    assert!(
        &v == json!({
            "x": 0,
            "y": 0,
            "z": 3,
        })
        .as_object()
        .unwrap()
    );

    Ok(())
}
