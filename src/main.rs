use rand::prelude::*;
use plotters::prelude::*;

fn sample_module<R: Rng>(rng: &mut R, ri: f64) -> u8 {
    let t: u8 = 27;
    if rng.gen::<f64>() < ri {
        t
    } else {
        let mut x: u8 = rng.gen_range(0..63);
        if x >= t { x += 1; }
        x
    }
}

fn classic_voter<R: Rng>(rng: &mut R, o: [u8; 3]) -> u8 {
    let a = o[0];
    let b = o[1];
    let c = o[2];
    if a == b || a == c { return a; }
    if b == c { return b; }
    o[rng.gen_range(0..3)]
}

fn map_voter(o: [u8; 3], rs: [f64; 3]) -> u8 {
    let mut best_v = o[0];
    let mut best_logp = f64::NEG_INFINITY;
    for &v in &o {
        let mut logp = 0.0;
        for i in 0..3 {
            if o[i] == v {
                logp += rs[i].ln();
            } else {
                logp += ((1.0 - rs[i]) / 63.0).ln();
            }
        }
        if logp > best_logp {
            best_logp = logp;
            best_v = v;
        }
    }
    best_v
}

fn draw_chart(
    path: &str,
    n: u64,
    seed: u64,
    rs: [f64; 3],
    classic_ok: u64,
    map_ok: u64
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let maxv = classic_ok.max(map_ok).max(1);
    let y_top = ((maxv as f64) * 1.25).ceil() as u64;

    let title = format!(
        "TMR Comparison (Odd Case) | R = ({:.1}, {:.1}, {:.1}) | true = 27 | N = {} | seed = {}",
        rs[0], rs[1], rs[2], n, seed
    );

    let mut chart = ChartBuilder::on(&root)
        .margin(30)
        .caption(title, ("sans-serif", 40))
        .x_label_area_size(90)
        .y_label_area_size(120)
        .build_cartesian_2d(0i32..2i32, 0u64..y_top)?;

    chart
        .configure_mesh()
        .x_desc("Voter Type")
        .y_desc("Count of output = 27")
        .x_labels(2)
        .y_labels(10)
        .x_label_formatter(&|x| {
            match *x {
                0 => "Classic".to_string(),
                1 => "MAP".to_string(),
                _ => "".to_string(),
            }
        })
        .label_style(("sans-serif", 26))
        .axis_style(&BLACK)
        .light_line_style(&BLACK.mix(0.1))
        .draw()?;

    let blue = RGBColor(0, 114, 189);
    let orange = RGBColor(217, 83, 25);

    chart.draw_series(std::iter::once(Rectangle::new(
        [(0, 0), (1, classic_ok)],
        blue.filled(),
    )))?;

    chart.draw_series(std::iter::once(Rectangle::new(
        [(1, 0), (2, map_ok)],
        orange.filled(),
    )))?;

    let classic_rate = classic_ok as f64 / n as f64 * 100.0;
    let map_rate = map_ok as f64 / n as f64 * 100.0;

    chart.draw_series(std::iter::once(Text::new(
        format!("{} ({:.2}%)", classic_ok, classic_rate),
        (0, classic_ok + y_top / 40),
        ("sans-serif", 28).into_font(),
    )))?;

    chart.draw_series(std::iter::once(Text::new(
        format!("{} ({:.2}%)", map_ok, map_rate),
        (1, map_ok + y_top / 40),
        ("sans-serif", 28).into_font(),
    )))?;

    root.draw(&Text::new(
        "Classic voter: majority with random tie-break",
        (60, 980),
        ("sans-serif", 26).into_font(),
    ))?;

    root.draw(&Text::new(
        "MAP voter: reliability-aware (uses Ri and uniform fault model)",
        (60, 1020),
        ("sans-serif", 26).into_font(),
    ))?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rs: [f64; 3] = [0.9, 0.5, 0.2];
    let t: u8 = 27;

    let args: Vec<String> = std::env::args().collect();
    let n: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(7);

    let mut rng = StdRng::seed_from_u64(seed);

    let mut classic_ok: u64 = 0;
    let mut map_ok: u64 = 0;

    for _ in 0..n {
        let o = [
            sample_module(&mut rng, rs[0]),
            sample_module(&mut rng, rs[1]),
            sample_module(&mut rng, rs[2]),
        ];
        if classic_voter(&mut rng, o) == t { classic_ok += 1; }
        if map_voter(o, rs) == t { map_ok += 1; }
    }

    println!("N={n} seed={seed}");
    println!("classic_ok={classic_ok} classic_rate={}", classic_ok as f64 / n as f64);
    println!("map_ok={map_ok} map_rate={}", map_ok as f64 / n as f64);

    draw_chart("TMR_Comparison.png", n, seed, rs, classic_ok, map_ok)?;
    Ok(())
}
