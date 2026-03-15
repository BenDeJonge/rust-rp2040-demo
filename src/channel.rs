use core::cell::Cell;

/// A rudimentary MPSC channel which will be used to set and read the button states.
pub struct Channel<T> {
    // Used for interior mutability such that multiple producers can set this.
    item: Cell<Option<T>>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            item: Cell::new(None),
        }
    }

    pub fn get_sender(&self) -> Sender<'_, T> {
        Sender { channel: self }
    }

    pub fn send(&self, item: T) {
        self.item.replace(Some(item));
    }

    pub fn get_receiver(&self) -> Receiver<'_, T> {
        Receiver { channel: self }
    }

    pub fn receive(&self) -> Option<T> {
        self.item.take()
    }
}

/// Can put a `T` on a `Channel`.
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(&self, item: T) {
        self.channel.send(item)
    }
}

/// Can get a `T` from a `Channel`.
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
    pub fn receive(&self) -> Option<T> {
        self.channel.receive()
    }
}
