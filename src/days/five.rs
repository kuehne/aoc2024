use std::collections::{HashMap, HashSet};

pub fn calculate(input: &str) -> i64 {
    let (rules, page_updates) = parse_file(input);
    page_updates
        .iter()
        .filter(|page_update| is_valid_page_update(page_update, &rules))
        .map(get_middle_value)
        .sum::<i64>()
}

pub fn calculate_variant(input: &str) -> i64 {
    let (rules, page_updates) = parse_file(input);
    page_updates
        .iter()
        .filter(|page_update| !is_valid_page_update(page_update, &rules))
        .map(|page_update| get_corrected_middle_value(page_update, &rules))
        .sum::<i64>()
}

fn parse_file(input: &str) -> (HashMap<i64, (HashSet<i64>, HashSet<i64>)>, Vec<Vec<i64>>) {
    let mut rules = HashMap::new();
    let mut pages = Vec::new();
    let mut is_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            is_rules = false;
            continue;
        }
        if is_rules {
            let values = line
                .split("|")
                .map(|l| l.parse::<i64>().expect("Not parseable"))
                .collect::<Vec<i64>>();
            add_before_rule((values[0], values[1]), &mut rules);
            add_after_rule((values[0], values[1]), &mut rules);
        } else {
            let values = line
                .split(",")
                .map(|l| l.parse::<i64>().expect("Not parseable"))
                .collect::<Vec<i64>>();
            pages.push(values);
        }
    }
    (rules, pages)
}

fn add_before_rule(values: (i64, i64), rules: &mut HashMap<i64, (HashSet<i64>, HashSet<i64>)>) {
    let rule_before = rules
        .entry(values.0)
        .or_insert((HashSet::new(), HashSet::new()));
    rule_before.0.insert(values.1);
}

fn add_after_rule(values: (i64, i64), rules: &mut HashMap<i64, (HashSet<i64>, HashSet<i64>)>) {
    let rule_after = rules
        .entry(values.1)
        .or_insert((HashSet::new(), HashSet::new()));
    rule_after.1.insert(values.0);
}

fn is_valid_page_update(
    page_update: &Vec<i64>,
    rules: &HashMap<i64, (HashSet<i64>, HashSet<i64>)>,
) -> bool {
    for (i, el) in page_update.iter().enumerate() {
        if !rules.contains_key(el) {
            continue;
        }
        let el_rules = rules.get(el).unwrap();
        if i > 0 && !is_valid_after(page_update[..i].to_vec(), &el_rules.0) {
            return false;
        }
        if (i + 1) != page_update.len() && !is_valid_before(page_update[i..].to_vec(), &el_rules.1)
        {
            return false;
        }
    }
    true
}

fn is_valid_before(before_elements: Vec<i64>, rules: &HashSet<i64>) -> bool {
    for before_element in before_elements.iter() {
        if rules.contains(before_element) {
            return false;
        }
    }
    true
}

fn is_valid_after(after_elements: Vec<i64>, rules: &HashSet<i64>) -> bool {
    for before_element in after_elements.iter() {
        if rules.contains(before_element) {
            return false;
        }
    }
    true
}

fn get_middle_value(page_update: &Vec<i64>) -> i64 {
    page_update
        .get(page_update.len() / 2)
        .expect("Could not get middle value")
        .clone()
}

fn get_corrected_middle_value(
    page_update: &Vec<i64>,
    rules: &HashMap<i64, (HashSet<i64>, HashSet<i64>)>,
) -> i64 {
    get_middle_value(&correct_page_update(
        page_update[0],
        &page_update[1..].to_vec(),
        rules,
    ))
}

fn correct_page_update(
    el: i64,
    page_update: &Vec<i64>,
    rules: &HashMap<i64, (HashSet<i64>, HashSet<i64>)>,
) -> Vec<i64> {
    if page_update.is_empty() {
        return vec![el];
    }
    let before_elements = page_update
        .iter()
        .filter(|&page_update| is_valid_before_el(el, *page_update, rules))
        .collect::<Vec<_>>();
    let after_elements = page_update
        .iter()
        .filter(|&page_update| !is_valid_before_el(el, *page_update, rules))
        .collect::<Vec<_>>();
    let mut res = correct_elements(&before_elements, rules);
    res.push(el);
    res.extend(correct_elements(&after_elements, rules));
    res
}

fn correct_elements(
    elements: &Vec<&i64>,
    rules: &HashMap<i64, (HashSet<i64>, HashSet<i64>)>,
) -> Vec<i64> {
    let mut res = Vec::new();
    if elements.len() == 1 {
        res.push(elements[0].clone());
    } else if elements.len() > 1 {
        res.extend(correct_page_update(
            elements[0].clone(),
            &elements[1..]
                .iter()
                .map(|&t| t.clone())
                .collect::<Vec<i64>>(),
            rules,
        ));
    }
    res
}

fn is_valid_before_el(
    el: i64,
    comp_el: i64,
    rules: &HashMap<i64, (HashSet<i64>, HashSet<i64>)>,
) -> bool {
    rules.get(&el).unwrap().0.contains(&comp_el) || rules.get(&comp_el).unwrap().1.contains(&el)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/05.txt").expect("Test file not found.")));
        assert_eq!(result, 143);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/05.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 123);
    }
}
