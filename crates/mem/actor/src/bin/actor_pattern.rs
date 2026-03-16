use tokio::sync::{mpsc, oneshot};

// The Message enum represents commands sent to the actor.
enum Message {
    UpdateLocation {
        driver_id: u32,
        lat: f64,
        lng: f64,
    },
    GetDriverStatus {
        driver_id: u32,
        respond_to: oneshot::Sender<Option<DriverStatus>>,
    },
}

#[derive(Debug, Clone)]
struct DriverStatus {
    driver_id: u32,
    lat: f64,
    lng: f64,
    update_count: u64,
}

// The Actor: a struct that owns data and lives inside a spawned task.
// No Arc<Mutex> needed—only one task accesses the data.
struct DriverActor {
    receiver: mpsc::Receiver<Message>,
    drivers: std::collections::HashMap<u32, DriverStatus>,
}

impl DriverActor {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            receiver,
            drivers: std::collections::HashMap::new(),
        }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::UpdateLocation {
                driver_id,
                lat,
                lng,
            } => {
                let status = self
                    .drivers
                    .entry(driver_id)
                    .or_insert(DriverStatus {
                        driver_id,
                        lat: 0.0,
                        lng: 0.0,
                        update_count: 0,
                    });
                status.lat = lat;
                status.lng = lng;
                status.update_count += 1;
            }
            Message::GetDriverStatus {
                driver_id,
                respond_to,
            } => {
                let status = self.drivers.get(&driver_id).cloned();
                // Ignore send errors if the caller dropped the receiver.
                let _ = respond_to.send(status);
            }
        }
    }
}

// The Handle: a clonable struct that holds an mpsc::Sender.
// This is what you pass around the application.
#[derive(Clone)]
struct DriverHandle {
    sender: mpsc::Sender<Message>,
}

impl DriverHandle {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let mut actor = DriverActor::new(receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    async fn update_location(
        &self,
        driver_id: u32,
        lat: f64,
        lng: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.sender
            .send(Message::UpdateLocation {
                driver_id,
                lat,
                lng,
            })
            .await?;
        Ok(())
    }

    async fn get_driver_status(
        &self,
        driver_id: u32,
    ) -> Result<Option<DriverStatus>, Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Message::GetDriverStatus {
                driver_id,
                respond_to: tx,
            })
            .await?;
        Ok(rx.await?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = DriverHandle::new();

    // Multiple clones can be sent to different tasks.
    let h1 = handle.clone();
    let h2 = handle.clone();

    let task1 = tokio::spawn(async move {
        h1.update_location(1, 40.7128, -74.0060).await.unwrap();
        h1.update_location(1, 40.7130, -74.0062).await.unwrap();
    });

    let task2 = tokio::spawn(async move {
        h2.update_location(2, 34.0522, -118.2437).await.unwrap();
    });

    task1.await?;
    task2.await?;

    let status = handle.get_driver_status(1).await?;
    println!("Driver 1: {:?}", status);

    let status = handle.get_driver_status(2).await?;
    println!("Driver 2: {:?}", status);

    let status = handle.get_driver_status(99).await?;
    println!("Driver 99: {:?}", status);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_actor_pattern() {
        let handle = DriverHandle::new();
        handle.update_location(1, 40.7128, -74.0060).await.unwrap();
        handle.update_location(1, 40.7130, -74.0062).await.unwrap();

        let status = handle.get_driver_status(1).await.unwrap().unwrap();
        assert_eq!(status.driver_id, 1);
        assert_eq!(status.update_count, 2);

        let missing = handle.get_driver_status(99).await.unwrap();
        assert!(missing.is_none());
    }
}
