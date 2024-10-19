//A function which accepts a function and spits out the time and the results


pub fn timeit<F>(&self, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);

    self.sender.as_ref().unwrap().send(job).unwrap();
}