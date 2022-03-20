use std::{sync::{Arc, mpsc,Mutex}, thread};
// use postfres::{Client, NoTls};
// let marchandise mut  = Client::connect("host = localhost user = postgres", NoTls)?;



pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,

}
type Merchandise = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewMerchandise(Merchandise),
    Terminate,
    
}

impl ThreadPool {
    pub fn new(size: usize)-> ThreadPool{
        assert!(size > 0);
        
        let(sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(
                id, 
                Arc::clone(&receiver)
            ));
        }
        ThreadPool{ workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce()+ Send + 'static
        {
            let merchandise = Box::new(f);
            self.sender.send(Message::NewMerchandise(merchandise)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Wocker terminate got a job; executing");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
            
        }

        for wocker in &mut self.workers{
            println!("Shutting down worker {}", wocker.id);

            if let Some(thread) = wocker.thread.take(){
                thread.join().unwrap();
            }

        }
    }
    
}

struct  Worker {
    id : usize,
    thread: Option<thread::JoinHandle<()>>

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move|| loop{
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
                
            match message {
                Message::NewMerchandise(merchandise) => {
                    println!("Worker {} got a job; executing.", id);
                    merchandise();
                    }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
                }

        });

        Worker {id, thread: Some(thread)}
    }
}

