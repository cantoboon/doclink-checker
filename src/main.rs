use git2::Repository;

fn main() {
    let repo = match Repository::open("/home/lewis/rust/simple") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repo: {}", e),
    };
}


/// Shows the greatest common donominator
fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b
    }
    if b == 0 {
        return a
    }
    if b == a {
        return b
    }

    if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}