mod permission;
mod role;
mod user;
mod user_id;

pub use self::permission::Permission;
pub use self::permission::PermissionError;
pub use self::role::Role;
pub use self::role::RoleError;
pub use self::user::User;
pub use self::user_id::UserId;
pub use self::user_id::UserIdError;

pub const USER_ID_TO_ROLES: std::cell::LazyCell<
    std::collections::BTreeMap<UserId, std::collections::BTreeSet<Role>>,
> = std::cell::LazyCell::new(|| {
    std::collections::BTreeMap::from_iter([
        (
            <UserId as std::str::FromStr>::from_str("admin123").expect("valid user id"),
            std::collections::BTreeSet::from_iter([Role::Admin]),
        ),
        (
            <UserId as std::str::FromStr>::from_str("user123").expect("valid user id"),
            std::collections::BTreeSet::from_iter([Role::User]),
        ),
    ])
});

pub const ROLE_TO_PERMISSIONS: std::cell::LazyCell<
    std::collections::BTreeMap<Role, std::collections::BTreeSet<Permission>>,
> = std::cell::LazyCell::new(|| {
    std::collections::BTreeMap::from_iter([
        (
            Role::Admin,
            std::collections::BTreeSet::from_iter([Permission::A, Permission::B, Permission::C]),
        ),
        (
            Role::User,
            std::collections::BTreeSet::from_iter([Permission::A]),
        ),
    ])
});
