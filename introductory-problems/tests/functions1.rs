// Don't mind this for now :)

fn call_me() {}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn call_function() {
        call_me();
    }

}