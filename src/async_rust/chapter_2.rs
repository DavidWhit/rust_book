fn under_the_hood_futures_and_tasks() {
/*
   /*
    The Future trait is at the center of async in Rust. A future is an async computation that
    can produce a value( although that value may be empty eg ()) A simplified version of the future
    trait might look something like this:
    */

    trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake:fn()) -> Poll<Self::Output>;
    }

    enum Poll<T> {
        Ready(T),
        Pending,
    }

    /*
    Futures can be advanced by calling the poll function, which will drive the future as far
    towards completion as possible. If the future completes it returns Poll::Ready(result)
    otherwise Poll::Pending and arranges for wake() function to be called when the future
    is ready to make progress. When wake() is called, the executor driving the Future will call
    poll again so that the future can make more progress.

    Without wake() the executor would have no way to knowing when a particular future could make
    progress, and would have to be constantly polling every future. With wake() the executor
    knows exactly which futures are ready to be polled.

    For example consider the case where we want to read from a socket that may or may not have
    data available already. If there is data, we can read it and return Poll::Ready(data), but if
    no data is ready our future is blocked and can no longer make progress. When no data is available
    we must register wake to be called when data becomes ready on the socket which will tell me
    the executor that our future is ready to make progress. A simple socketread future
    might look something like this:
    */

    pub struct SocketRead<'a> {
        socket: &'a Socket
    }

    impl SimpleFuture for SocketRead<'_> {
        type Output = Vec<u8>;

        fn poll(&mut self, wake:fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                // The socket has data-- read it into a buffer and return it
                Poll::Ready(self.socket.read_buf())
            } else {
                // The socket doesnt have data yet

                // Arrange for a wake to be called once data is available
                self.socket.set_readable_callback(wake);
                Poll::Pending
            }
        }
    }

    // This model of future allows for composing together multiple asynchronous operations
    // without needing intermediate allocations. Running multiple futures at once or chaining
    // futures together can be implemented via alloction-free state machines like this:
    // (REMEMBER async fn's create state machines underneath)

    pub struct Join<FutureA, FutureB> {
        a: Option<FutureA>,
        b: Option<FutureB>
    }

    impl <FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>, // This is also example of trait type getting assigned a concrete type
        FutureA: SimpleFuture<Output = ()>,
    {
        type Output = ();

        fn poll(&mut self, wake:fn()) -> Poll<Self::Output> {
            // Attempt to complete future 'a'
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }

            // Attempt to complete future 'b'
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // both futures have completed
                Poll::ready(())
            } else {
                // one or both are still pending and still have work to do
                // they will call wake when progress can be made
                Poll::Pending
            }
        }
    }

    /*
    This shows how multiple futures can be run simultaneously without needing separate allocations
    allowing for more efficient asynchronous programs. Similarly, multiple sequential futures
    can be run one after another like this:
    */

    pub struct AndThenFut<FutureA, FutureB> {
        first: Opton<FutureA>,
        second: FutureB
    }

    impl <FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if let Some(first) = &mut self.first {
                match first.poll(wake) {
                    // Completed the first future remove it and start on the second
                    Poll::Ready(()) => self.first.take(),
                    // couldn't yet complete the first future
                    Poll::Pending => return Poll::Pending
                }
            }
        }

        // now that the first future is done, attempt to complete the second.
        self.second.poll(wake)
    }

    /*
    These examples show how the Future trait can be used to express asynchronous control flow
    without requiring multiple allocated objects and deeply nested callbacks. With the basic
    control-flow out of the way lets talk about the real Future trait and how it is different
     */

    trait Future {
        type Output;
        fn poll(
            // note the change from &mut self to Pin<&mut Self>
            self: Pin<&mut Self>,
            // and the change from wake: fn() to cx &mut Context<'_>
            cx: &mut Context<'_>) -> Poll<Self::Output>;
    }

    /*
     Pin is the first main change we'll discuss later; for now, just know that it allows us
     to create futures that are immovable. Immovable objects cant store pointers between their
     fields eg struct MyFut { a: i32, ptr_to_a: *const i32} Pinning is necessary to enable
     async/await.

     Secondly wake: fn() changed to &mut Context<'_> In SimpleFuture we used a call to a function
     pointer(fn()) to tell the future executor the future in question should be polled. However,
     since fn() is just a function pointer it can't store any data about which Future called wake

     In a real world scenario a complex application like a web server may have thousands of
     different connections whose wakeups should all be managed separately. The Context type solves
     this by providing access to a value of type Waker, which can be used to wake a specific task.
     */
*/
}

use {
    futures:: {
        future::{ BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::{Future},
        pin::Pin,
        sync::{Arc,Mutex},
        sync::mpsc::{sync_channel, Receiver, SyncSender },
        task::{Context, Poll, Waker },
        thread,
        time::Duration,
    }
};

pub fn task_wakeups_with_waker() {
    /*
    common that the first time futures are polled aren't able to complete.
    When that happends the future needs to ensure that it is polled again once it is ready to make
    more progress. this is don with the Waker type.

    Each time a future is polled it is polled as a part of a task.Tasks are just top-level futures
    that have been submitted to an executor

    Waker provides a wake() method that can be used to tell the executor that the associated task
    should be awakened. When wake() is called the executor knows that the task associated with the
    Waker is ready to make progress, and its future should be polled again.

    Waker also implements clone() so that it can be copied around and stored.

    Lets try implementing a simple timer future using Waker

     */

    pub struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>
    }

    struct SharedState {
        completed:bool,
        waker:Option<Waker>
    }

    impl Future for TimerFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut shared_state = self.shared_state.lock().unwrap();
            if shared_state.completed {
                Poll::Ready(())
            } else {
                /*
                It's tempting to do this once vs clone however, the timerfuture can move between
                executors which could cause a stale wakre pointing to the wrong task
                preventing the Timerfuture from waking up correctly

                however it is possible to check for this using Waker::will_wake but it is omitted
                for simplicity
                 */
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
            }
        }
    }

    /*
    Importantly we have to update the Waker every time the future is polled because the future may
    have moved to a different task with a different Waker. This will happen when futures are passed
    around between tasks after being polled.
     */

    // Finally create an API to actually construct the timer and start a thread

    impl TimerFuture {
        pub fn new(duration:Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState { completed: false, waker: None}));

            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            });

            TimerFuture { shared_state }
        }
    }
    // Now an executor

    /*
    This will work by sending tasks to run over a channel. The executor will pull events off of the
    channel and run them. When a task is ready to do more work (is awakened) it can schedule itself
    to be polled again by putting itself back onto the channel.

     In this desing, the executor itself just needs receiving end of the task channel. The user will
     get a sending end so that they can spawn new futures. Tasks themselves are just futures that can
     reschedule themselves, so we'll store them as a future paired with a sender that the task can use
     to requeue itself
    */

    // Task executor that receives tasks off of a channel and runs them.
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    // When a waker is created from an Arc<Task> calling wake() on it will cause a copy of the Arc
    // to be sent onto the task channel. Our executor then needs to pick up the task and poll it.

    impl Executor {
        fn run(&self) {
            while let Ok(task) = self.ready_queue.recv() {
                // Take the future and if it has not yet completed (is still some)

                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);

                    if let Poll::Pending = future.as_mut().poll(context) {
                        // Not done put it back in its task to be run again
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }

    // 'spawner' spawns new futures onto the task channel
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>
    }

    // Add a spawner to make it easy to spawn new futures
    // taking a box future and create a new task

    impl Spawner {
        fn spawn(&self, future:impl Future<Output = ()> + 'static + Send) {
           let future = future.boxed();
            let task = Arc::new(Task { future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone()});
            self.task_sender.send(task).expect("too many tasks queued");
        }
    }

    // A future that can reschedule self to be polled by an Executor
    struct Task {
        /*
        In progress future that should be pushed to completion.

        The mutex is not necessary for correctness since we only have one thread executing tasks at once.
        However Rust isnt smart enough to know that future is only mutated from one thread. So we need to
        use Mutex to prove thread-safety. A production executor would not need this and could use
        unsafeCell instead.
        */

        future: Mutex<Option<BoxFuture<'static,() >>>,
        // Handle to place the task itself back onto the task queue
        task_sender: SyncSender<Arc<Task>>
    }

    // To poll futures we need to create a Waker
    // recall wakers are responsible for scheduling a task to be polled again once wake is called.
    // Remember that Wakers tell the executor exactly which task has become ready
    // allowing them to poll just the futures that are ready to make progress
    // The easiest way is by using ArcWake trait and then using the waker_ref or into_waker() func
    // to turn an Arc<impl ArcWake> into a Waker

    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            let cloned = arc_self.clone();
            arc_self
                .task_sender
                .send(cloned)
                .expect("too many tasks queued")
        }
    }

    fn new_executor_and_spawner() -> (Executor, Spawner)  {
        // Max number of tasks allow queueing in the channel at once

        // this is just to make the sync_channel happy and would be in a real executor
        const MAX_QUEUED_TASKS:usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor {ready_queue}, Spawner { task_sender} )
    }


    let (executor, spawner) = new_executor_and_spawner();


    // spawn a task to print before and after waiting on a timer
    spawner.spawn(async {
        println!("howdy!");
        // wait
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);
    executor.run();

}