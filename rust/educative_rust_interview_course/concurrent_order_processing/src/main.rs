use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{sleep, Duration};

// Order structure with additional fields if necessary
struct Order {
    id: u64,
    // Add additional fields as necessary
}

enum OrderStatus {
    Validated,
    Paid,
    Shipped,
}

enum OrderError {
    ValidationError,
    PaymentError,
    ShipmentError,
}

struct OrderProcessor {
    max_concurrent_orders: usize,
    max_payment_concurrency: usize,
    payment_semaphore: Arc<Mutex<usize>>,
    results: Arc<Mutex<VecDeque<Result<OrderStatus, OrderError>>>>,
}

impl OrderProcessor {
    fn new(max_concurrent_orders: usize, max_payment_concurrency: usize) -> Self {
        OrderProcessor {
            max_concurrent_orders,
            max_payment_concurrency,
            payment_semaphore: Arc::new(Mutex::new(max_payment_concurrency)),
            results: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    async fn process_order(&self, order: &Order) -> Result<OrderStatus, OrderError> {
        // Validate Order
        match Self::validate_order(&order).await {
            Ok(status) => {
                // Process Payment
                match self.process_payment(&order).await {
                    Ok(_) => {
                        // Ship Order
                        match Self::ship_order(&order).await {
                            Ok(status) => Ok(status),
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn validate_order(order: &Order) -> Result<OrderStatus, OrderError> {
        // Simulate validation
        sleep(Duration::from_millis(100)).await;
        Ok(OrderStatus::Validated)
    }

    async fn process_payment(&self, order: &Order) -> Result<OrderStatus, OrderError> {
        let _permit = self.payment_semaphore.lock().unwrap();

        // Simulate payment processing
        sleep(Duration::from_millis(200)).await;

        Ok(OrderStatus::Paid)
    }

    async fn ship_order(order: &Order) -> Result<OrderStatus, OrderError> {
        // Simulate shipment
        sleep(Duration::from_millis(150)).await;
        Ok(OrderStatus::Shipped)
    }
}

#[tokio::main]
async fn main() {
    let orders = vec![
        Order { id: 1 },
        Order { id: 2 },
        Order { id: 3 },
        Order { id: 4 },
        Order { id: 5 },
        // Add more orders as needed
    ];

    let processor = OrderProcessor::new(10, 3);

    let mut handles = vec![];
    let processor = Arc::new(processor);

    for order in &orders {
        let processor_shared = processor.clone();
        handles.push(task::spawn(async move {
            let result = processor_shared.process_order(order).await;
            let mut results_lock = processor_shared.results.lock().unwrap();
            results_lock.push_back(result);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let results_lock = processor.results.lock().unwrap();
    let results: Vec<_> = results_lock.into_iter().collect();

    for result in results {
        match result {
            Ok(status) => match status {
                OrderStatus::Validated => println!("Order Validated"),
                OrderStatus::Paid => println!("Order Paid"),
                OrderStatus::Shipped => println!("Order Shipped"),
            },
            Err(e) => match e {
                OrderError::ValidationError => println!("Order Validation Failed"),
                OrderError::PaymentError => println!("Order Payment Failed"),
                OrderError::ShipmentError => println!("Order Shipment Failed"),
            },
        }
    }
}
