fn main() {
    // Automatically trigger recompilation if a migration changes
    // https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#triggering-recompilation-on-migration-changes
    println!("cargo:rerun-if-changed=migrations");
}
