pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        //On my machine: usize=64 bits/8 bytes
        // 8 * 3 (length, capacity, pointer)
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        //On my machine: usize=64 bits/8 bytes for each field (title, description, title)
        // 8 bytes * 3 * 3(length, capacity, pointer)
        assert_eq!(size_of::<Ticket>(), 72); 
    }
}
