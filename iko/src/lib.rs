#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        trait Migration {
            fn migrate(&self);
            fn version(&self) -> u32;
        }

        struct Migration1 {}
        impl Migration for Migration1 {
            fn migrate(&self) {
                println!("migrate1");
            }

            fn version(&self) -> u32 {
                1
            }
        }

        struct Migration2 {}
        impl Migration for Migration2 {
            fn migrate(&self) {
                println!("migrate2");
            }

            fn version(&self) -> u32 {
                2
            }
        }

        let mut current_version = 0;
        let migrations: Vec<Box<dyn Migration>> =
            vec![Box::new(Migration1 {}), Box::new(Migration2 {})];
        for migration in migrations {
            // load_and_lock_current_version()
            if current_version + 1 != migration.version() {
                continue;
            }

            migration.migrate();

            // save_and_unlock_current_version()
            current_version += 1;
        }
    }
}
