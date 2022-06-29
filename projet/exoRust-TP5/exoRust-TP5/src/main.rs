use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct compute {
    number: i32,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fact/compute").post(compute);
    app.at("/fact/compute2/:n").get(compute2);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn compute(mut req: Request<()>) -> tide::Result {
    let compute { number } = req.body_json().await?;
    let mut tmpnumber = 1;
    for i in 1..number + 1 {
        tmpnumber *= i
    }
    Ok(format!("La factorielle du nombre {} est {}", number, tmpnumber).into())
}

async fn compute2(req: Request<()>) -> tide::Result<String> {
    let n: usize = req.param("n")?.parse().unwrap_or(0);
    let mut tmpnumber = 1;
    for i in 1..n + 1 {
        tmpnumber *= i
    }
    let res = format!("La factorielle du nombre {} est {}\n", n, tmpnumber);
    Ok(res)
}
