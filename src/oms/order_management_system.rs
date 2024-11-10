use crossbeam::channel::Receiver;
use crate::fix::{fix_message::FixMessage, messages::new_order::NewOrder};

pub struct OrderManagementSystem {
    receiver: Receiver<FixMessage>,
}

impl OrderManagementSystem {
    pub fn new(receiver: Receiver<FixMessage>) -> Self {
        OrderManagementSystem { receiver }
    }

    pub fn listen_for_orders(&self) {
        println!("Order Management System is now listening for messages...");

        while let Ok(message) = self.receiver.recv() {
            match message {
                FixMessage::NewOrder(order) => {
                    self.process_new_order(order);
                }
                FixMessage::ExecutionReport(report) => {
                    println!("Received ExecutionReport: {:?}", report);
                }
                FixMessage::OrderCancelRequest(request) => {
                    println!("Received OrderCancelRequest: {:?}", request);
                }
                FixMessage::OrderStatusRequest(status_request) => {
                    println!("Received OrderStatusRequest: {:?}", status_request);
                }
                FixMessage::Unknown => {
                    println!("Received an Unknown message type");
                }
            }
        }
    }

    fn process_new_order(&self, order: NewOrder) {
        println!("Processing new order: {:?}", order);
        // logic to follow ...
    }
}