use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Row<'a> {
    #[allow(dead_code)]
    barcode: &'a str,
    cluster: i16,
}

fn read_csv(file_path: &str) -> Result<Vec<i16>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let hdrs = rdr.headers()?.clone();
    let mut clusters = vec![];
    let mut raw_record = csv::StringRecord::new();
    while rdr.read_record(&mut raw_record)? {
        let row: Row = raw_record.deserialize(Some(&hdrs))?;
        clusters.push(row.cluster);
    }
    Ok(clusters)
}

fn rand_index(x: &[i16], y: &[i16]) -> f64 {
    assert!(x.len() == y.len());
    let n = x.len();
    let mut num = 0usize;
    for i in 0..n {
        for j in (i + 1)..n {
            let xi_eq_xj = x[i] == x[j];
            let yi_eq_yj = y[i] == y[j];
            if xi_eq_xj == yi_eq_yj {
                num += 1;
            }
        }
    }
    let den = n * (n - 1) / 2;
    (num as f64 / den as f64).min(1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let x = args.next().unwrap();
    let y = args.next().unwrap();
    let x_clusters = read_csv(&x)?;
    let y_clusters = read_csv(&y)?;
    let ri = rand_index(&x_clusters, &y_clusters);
    println!("{}", ri);
    Ok(())
}
