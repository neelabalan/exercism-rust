pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from("");
    }
    let mut proverb_list = vec![];
    for val in 0..list.len()-1 {
        proverb_list.push(format!("For want of a {} the {} was lost.", list[val], list[val+1]));
    }
    proverb_list.push(
        format!("And all for the want of a {}.", list[0])
    );
    return proverb_list.join("\n");
}

// ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]
/*
For want of a nail the shoe was lost.
For want of a shoe the horse was lost.
For want of a horse the rider was lost.
For want of a rider the message was lost.
For want of a message the battle was lost.
For want of a battle the kingdom was lost.
And all for the want of a nail.
 */