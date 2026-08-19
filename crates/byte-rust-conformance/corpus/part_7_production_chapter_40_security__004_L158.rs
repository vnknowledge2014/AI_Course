struct AccessRequest {
    user_id: u64,
    resource_owner_id: u64,
    action: String, // "edit"
}

// Hàm Policy đánh giá theo Logic thay vì Roles
fn evaluate_policy(req: &AccessRequest) -> Result<(), String> {
    if req.action == "edit" && req.user_id != req.resource_owner_id {
        return Err("Bạn chỉ được sửa tài nguyên của chính mình!".into());
    }
    Ok(())
}

fn main() {}
