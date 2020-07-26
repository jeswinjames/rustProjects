use std::thread;
use std::sync::{mpsc, mpsc::Sender, mpsc::Receiver, Arc, Mutex};

pub struct ThreadPool{
    _workers: Vec<Worker>,
    sender: Sender<Message>

}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _workers = Vec::with_capacity(size);
        for id in 0..size{
            _workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{ _workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static{
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();   
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        
        println!("Sending Terminate to all workers");
        for _ in &self._workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in & mut self._workers{
            println!("Waiting for worker {} to die", worker.id);
            if let Some(thread) = worker.thread.take(){
              thread.join().unwrap();  
            }
        }
    }
}

enum Message{
    NewJob(Job),
    Terminate
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker{
        let thread = thread::spawn( move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message{
                Message::NewJob(job) =>{
                    println!("Worker {} got a job. Executing.....", id);
                    job();
                }
                Message::Terminate =>{
                    println!("Received terminate. Killing worker {}...", id);
                    break;
                }
            }
            
        });
        Worker{ id,
            thread : Some(thread)}
    }
}