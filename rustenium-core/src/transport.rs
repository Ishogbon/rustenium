pub trait ConnectionTransport {
    fn send(&self, message: String) -> ();
    fn on_message(&self, message: String) -> ();
    fn close(&self) -> ();
    fn on_close(&self) -> ();
}
