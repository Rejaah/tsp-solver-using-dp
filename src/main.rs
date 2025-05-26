use std::io::{BufRead};
use std::time::Instant;

const INF: i32 = 1_000_000_000;
fn main() {
    clear_screen();

    println!("Traveling Salesman Problem (TSP) Solver Menggunakan Dynamic Programming");
    println!("Masukkan ukuran matriks (banyak kota) dan matriks adjacency:");

    let stdin = std::io::stdin();
    let mut lines  = stdin.lock().lines();

    // baca ukuran matriks nxn (banyak kota)
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Harus memasukkan angka"),
        _ => panic!("Gagal membaca ukuran matriks"),
    };

    // baca matriks adjacency
    let mut m  = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().expect("Baris kosong").expect("Gagal membaca baris");
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Harus berupa angka"))
            .collect();
        m.push(row);
    }
    
    // jalankan TSP solver
    let start = Instant::now();
    let (min_cost, path) = tsp_solver(n, &m);
    let duration = start.elapsed();

    println!("\nBiaya minimum: {}", min_cost);
    print!("Rute perjalanan: ");
    for (i, city) in path.iter().enumerate() {
        if i > 0 {
            print!(" → ");
        }
        print!("{}", city);
    }
    println!("\nWaktu eksekusi: {:.4?}", duration);
}

fn tsp_solver(n: usize, m: &[Vec<i32>]) -> (i32, Vec<usize>) {
    // jumlah subset maksimal adalah 2^n
    let max_mask = 1 << n;

    // dp[i][j] menyimpan jarak minimum untuk mengunjungi subset i berakhir di kota j
    let mut dp = vec![vec![INF; n]; max_mask];

    // vector untuk menyimpan rute
    let mut parent = vec![vec![None; n]; max_mask];
    
    // mulai dari kota 0, hanya kota 0 yang dikunjungi
    dp[1][0] = 0; 

    for mask in 0..max_mask {
        if mask & 1 == 0 {
            continue;
        }

        for last in 0..n {
            if mask & (1 << last) == 0 {
                continue;
            }

            for next in 0..n {
                if mask & (1 << next) != 0 {
                    continue;
                }
                let next_mask = mask | (1 << next);
                let cost = dp[mask][last] + m[last][next];
                if cost < dp[next_mask][next] {
                    dp[next_mask][next] = cost;
                    parent[next_mask][next] = Some(last);
                }
            }
        }
    }

    // kembali ke kota 0
    let full_mask = (1 << n) - 1;
    let mut min_cost = INF;
    let mut last_city = 0;
    for last in 1..n {
        let cost = dp[full_mask][last] + m[last][0];
        if cost < min_cost {
            min_cost = cost;
            last_city = last;
        }
    }

    // rekonstruksi rute
    let mut path = vec![0; n + 1];
    let mut mask = full_mask;
    let mut current = last_city;

    for i in (1..n).rev() {
        path[i] = current;
        let prev = parent[mask][current].unwrap();
        mask ^= 1 << current;
        current = prev;
    }

    path[0] = 0; // mulai dari 0
    path[n] = 0; // kembali ke 0

    (min_cost, path)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}