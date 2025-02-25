use std::io::{stdin, stdout, BufWriter, Write};

struct Student {
    name: String,
    age: u32,
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<u32>().unwrap();

    let mut v: Vec<Student> = Vec::new();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let (age, name): (u32, String) = {
            let v = input.split_whitespace().collect::<Vec<_>>();

            (v[0].trim().parse::<u32>().unwrap(), v[1].to_string())
        };

        let student = Student { age, name };

        v.push(student);
    }

    v.sort_by(|a, b| a.age.cmp(&b.age));

    for i in v.iter() {
        writeln!(out, "{} {}", i.age, i.name).unwrap();
    }
}
