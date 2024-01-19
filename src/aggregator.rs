pub async fn run_aggregator(mut receiver: tokio::sync::mpsc::UnboundedReceiver<f64>, num_clients: usize) {
    let mut total_average = 0.0;
    let mut count = 0;

    // Receive values from all clients
    while let Some(average_price) = receiver.recv().await {
        total_average += average_price;
        count += 1;

        // Exit loop when all clients have sent their values
        if count == num_clients {
            break;
        }
    }

    // Calculate the final average
    let overall_average = if count > 0 {
        total_average / count as f64
    } else {
        0.0
    };

    println!("Aggregator complete. The overall average USD price of BTC is: {}", overall_average);
}
