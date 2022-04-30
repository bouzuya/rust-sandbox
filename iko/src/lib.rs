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

        struct Migrator {
            current_version: u32,
        }
        impl Migrator {
            fn load_and_lock_current_version(&self) -> u32 {
                self.current_version
            }

            fn save_and_unlock_current_version(&mut self, new_current_version: u32) {
                self.current_version = new_current_version;
            }
        }

        let mut migrator = Migrator { current_version: 0 };

        let migrations: Vec<Box<dyn Migration>> =
            vec![Box::new(Migration1 {}), Box::new(Migration2 {})];
        for migration in migrations {
            let current_version = migrator.load_and_lock_current_version();
            if current_version >= migration.version() {
                continue;
            }

            migration.migrate();

            migrator.save_and_unlock_current_version(migration.version());
        }
    }
}
