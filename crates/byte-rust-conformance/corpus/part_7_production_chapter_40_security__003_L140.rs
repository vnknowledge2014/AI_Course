enum Role { Admin, Editor, Viewer }
enum Permission { DeletePost, WritePost, ReadPost }

// Pure function ánh xạ Role -> Permissions
fn get_permissions(role: &Role) -> Vec<Permission> {
    match role {
        Role::Admin => vec![Permission::DeletePost, Permission::WritePost, Permission::ReadPost],
        Role::Editor => vec![Permission::WritePost, Permission::ReadPost],
        Role::Viewer => vec![Permission::ReadPost],
    }
}

fn main() {}
