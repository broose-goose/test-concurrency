use std::sync::mpsc::{RecvError, sync_channel};
use std::thread;

pub fn std_mpsc(iterations: usize) -> bool {
    // uses std sync primitives to
    let (sender, receiver) = sync_channel(5);
    let receive_handler = thread::spawn(move|| {
        let mut receive_count: usize = 0;
        loop {
            match receiver.recv() {
                Ok(_) => {
                    receive_count += 1;
                }
                Err(RecvError) => {
                    break receive_count;
                }
            }
        }
    });
    let send_handler = thread::spawn(move|| {
        for x in 0..iterations {
            if sender.send(x).is_err() {
                // If the receiver has disconnected, return an error
                return -1;
            }
        }
        0
    });
    let res_sender = send_handler.join().unwrap();
    let res_receiver = receive_handler.join().unwrap();

    res_sender == 0 && res_receiver == iterations
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_lib() {
        // make sure it works
        let res_1 = std_mpsc(1);
        assert!(res_1);
        // put it through some paces
        let res_2 = std_mpsc(50);
        assert!(res_2);
        // go for glory
        let res_3 = std_mpsc(5_000_000);
        assert!(res_3);
    }
}
