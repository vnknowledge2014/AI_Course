// V2 roles với permissions
#[derive(Debug, Clone, PartialEq)]
enum UserRole { Admin, Moderator, User }

#[derive(Debug, Clone, PartialEq)]
enum Permission {
    ReadContent,
    WriteContent,
    DeleteContent,
    ManageUsers,
    ViewAnalytics,
    ModerateComments,
}

impl UserRole {
    fn default_permissions(&self) -> Vec<Permission> {
        match self {
            UserRole::Admin => vec![
                Permission::ReadContent, Permission::WriteContent,
                Permission::DeleteContent, Permission::ManageUsers,
                Permission::ViewAnalytics, Permission::ModerateComments,
            ],
            UserRole::Moderator => vec![
                Permission::ReadContent, Permission::WriteContent,
                Permission::ModerateComments,
            ],
            UserRole::User => vec![
                Permission::ReadContent,
            ],
        }
    }
}

struct User {
    name: String,
    role: UserRole,
    extra_permissions: Vec<Permission>,
}

impl User {
    fn has_permission(&self, perm: &Permission) -> bool {
        self.role.default_permissions().contains(perm)
            || self.extra_permissions.contains(perm)
    }

    // Migration: old is_admin() → delegates to permission
    #[deprecated(note = "Use has_permission(Permission::ManageUsers)")]
    fn is_admin(&self) -> bool {
        self.has_permission(&Permission::ManageUsers)
    }
}

fn main() {
    let admin = User {
        name: "Minh".into(), role: UserRole::Admin,
        extra_permissions: vec![],
    };
    let moderator = User {
        name: "Lan".into(), role: UserRole::Moderator,
        extra_permissions: vec![Permission::ViewAnalytics],
    };

    println!("{}: manage_users={}, moderate={}",
        admin.name,
        admin.has_permission(&Permission::ManageUsers),
        admin.has_permission(&Permission::ModerateComments));

    println!("{}: manage_users={}, analytics={}",
        moderator.name,
        moderator.has_permission(&Permission::ManageUsers),
        moderator.has_permission(&Permission::ViewAnalytics));
}
