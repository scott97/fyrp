use heapless::spsc::Queue;
use heapless::consts::*;

fn main() {
    let mut queue: Queue<f32, U256, _> = Queue::u16();
    let (mut producer, mut consumer) = queue.split();

    producer.enqueue(1.0);
    producer.enqueue(2.0);
    producer.enqueue(3.0);
    producer.enqueue(4.0);
    producer.enqueue(5.0);
    producer.enqueue(6.0);
    producer.enqueue(7.0);
    producer.enqueue(8.0);
    producer.enqueue(9.0);
    producer.enqueue(10.0);


    assert_eq!(Some(1.0),consumer.dequeue());
    assert_eq!(Some(2.0),consumer.dequeue());
    assert_eq!(Some(3.0),consumer.dequeue());
    assert_eq!(Some(4.0),consumer.dequeue());
    assert_eq!(Some(&5.0),consumer.peek());
    assert_eq!(Some(&6.0),consumer.peek());


    assert_eq!(Some(5.0),consumer.dequeue());
    assert_eq!(Some(6.0),consumer.dequeue());
    assert_eq!(Some(7.0),consumer.dequeue());
    assert_eq!(Some(8.0),consumer.dequeue());
    assert_eq!(Some(&9.0),consumer.peek());
    assert_eq!(Some(&10.0),consumer.peek());
}


    
    

