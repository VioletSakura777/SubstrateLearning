fn foo<T>(items: &mut [T])
where
    T: PartialOrd + Copy,
{
    let len = items.len();

    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if items[j] > items[j + 1] {
                let temp = items[j];
                items[j] = items[j + 1];
                items[j + 1] = temp;
            }
        }
    }
}

fn main() {
    let mut items1 = [1, 94, 97, 6, 7, 5, 8, 9];

    let mut items2 = [(1, 2), (99, 8), (8, 7), (5, 99), (4, 1)];
    foo(&mut items1);
    foo(&mut items2);
    println!("{:?}", items1);
    println!("{:?}", items2);
}
