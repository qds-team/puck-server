#[derive(Debug, Serialize, Deserialize)]
struct MyClaims {
    ip_address: String,
}

impl Claims for MyClaims {}
