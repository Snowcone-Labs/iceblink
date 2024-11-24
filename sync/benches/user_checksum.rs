use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iceblink_sync::models::codes::Code;
use iceblink_sync::models::user::User;
use iceblink_sync::utils;

fn criterion_checksum(c: &mut Criterion) {
    let user = User {
        avatar_url: "https://github.com/Erb3.png".to_string(),
        display_name: "Erb3".to_string(),
        id: utils::generate_id(16usize),
        upstream_userid: utils::generate_id(16usize),
        username: "erb3".to_string(),
    };

    let mut codes: Vec<Code> = vec![];
    for _ in 0..20 {
        codes.push(Code {
            id: utils::generate_id(16usize),
            owner_id: user.id.clone(),
            content: utils::generate_id(16usize),
            display_name: utils::generate_id(16usize),
            icon_url: Some(utils::generate_id(16usize)),
            website_url: Some(utils::generate_id(16usize)),
        })
    }

    c.bench_function("checksum 20 codes", |b| {
        b.iter(|| utils::checksum(black_box(codes.clone()), black_box(&user)))
    });
}

criterion_group!(benches, criterion_checksum);
criterion_main!(benches);
