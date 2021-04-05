fn main() {
    // Initialize node
    rosrust::init("listener");

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let _subscriber_raii = rosrust::subscribe("chatter", 100, |v: rosrust_msg::std_msgs::UInt64| {
        // Callback for handling received messages
        rosrust::ros_info!("Received: {}", v.data);
    }).unwrap();

    // Block the thread until a shutdown signal is received
    rosrust::spin();
}