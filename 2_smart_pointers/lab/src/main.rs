// define here your structs


fn main() {
    linked_list_lab(&generate_number());
    double_linked_list_lab(&generate_number());
}

fn linked_list_lab(input: &Vec<usize>) -> usize{

    0
}

fn double_linked_list_lab(input: &Vec<usize>) -> usize {

    0
}



// ================================================================================================
use rand::Rng;
fn generate_number() -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let values: Vec<usize> = (0..10)
        .map(|_| { rng.gen_range(1..=10) })
        .collect();

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list() {
        let input = generate_number();
        assert_eq!(linked_list_lab(&input), input.iter().sum::<usize>());
    }

    #[test]
    fn double_linked_list() {
        let input = generate_number();
        let mut input2 = input.clone();
        input2.pop();

        assert_eq!(double_linked_list_lab(&input), input.iter().sum::<usize>() + input2.iter().sum::<usize>());
    }
}