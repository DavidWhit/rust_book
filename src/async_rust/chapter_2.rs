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

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc,Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
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
        shared_state: Arc<Mutex<SharedStated>>
    }

    struct SharedState {
        completed:bool,
        waker:Option<Waker>
    }

    impl Future for TimerFuture {
        type Output = ();

        fn poll(self: Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
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
}