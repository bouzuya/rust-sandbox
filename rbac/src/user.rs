use std::collections::BTreeSet;

use crate::Permission;
use crate::Role;
use crate::UserId;

pub struct User {
    pub id: UserId,
    pub permissions: BTreeSet<Permission>,
    pub roles: BTreeSet<Role>,
}
