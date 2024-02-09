#[derive(Clone)]
pub struct Card {
    suit: char,
    value: u8,
}

impl Card {
    fn from_str(s: &str) -> Card {
        //10S 10 of spades
        let suit = s.chars().last().unwrap();
        let value = match s.chars().nth(0).unwrap() {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            _ => s.chars().nth(0).unwrap().to_digit(10).unwrap() as u8,
        };
        Card { suit, value }
    }
}

pub fn rank_cards(cards: &[Card]) -> u8 {
    let mut sorted_cards = cards.to_vec();

    sorted_cards.sort_by(|b, a| a.value.cmp(&b.value));

    // Check for flush
    let is_flush = sorted_cards
        .windows(2)
        .all(|pair| pair[0].suit == pair[1].suit);

    // Check for straight
    let is_straight = sorted_cards
        .windows(2)
        .all(|pair| pair[0].value + 1 == pair[1].value)
        || (sorted_cards[0].value == 2
            && sorted_cards[1].value == 3
            && sorted_cards[2].value == 4
            && sorted_cards[3].value == 5
            && sorted_cards[4].value == 14); // Ace-low straight

    // Check for straight flush
    if is_straight && is_flush {
        return 8;
    }

    // check for four of a kind
    for i in 0..sorted_cards.len() - 3 {
        if sorted_cards[i].value == sorted_cards[i + 1].value
            && sorted_cards[i].value == sorted_cards[i + 2].value
            && sorted_cards[i].value == sorted_cards[i + 3].value
        {
            return 7;
        }
    }

    // check for full house
    if sorted_cards[0].value == sorted_cards[1].value
        && sorted_cards[2].value == sorted_cards[3].value
        && sorted_cards[2].value == sorted_cards[4].value
    {
        return 6;
    }

    if sorted_cards[0].value == sorted_cards[1].value
        && sorted_cards[0].value == sorted_cards[2].value
        && sorted_cards[3].value == sorted_cards[4].value
    {
        return 6;
    }

    // check for flush
    if is_flush {
        return 5;
    }

    // check for straight
    if is_straight {
        return 4;
    }

    // check for three of a kind
    for i in 0..sorted_cards.len() - 2 {
        if sorted_cards[i].value == sorted_cards[i + 1].value
            && sorted_cards[i].value == sorted_cards[i + 2].value
        {
            return 3;
        }
    }

    // check for two pair
    if sorted_cards[0].value == sorted_cards[1].value
        && sorted_cards[2].value == sorted_cards[3].value
    {
        return 2;
    }

    if sorted_cards[0].value == sorted_cards[1].value
        && sorted_cards[3].value == sorted_cards[4].value
    {
        return 2;
    }

    if sorted_cards[1].value == sorted_cards[2].value
        && sorted_cards[3].value == sorted_cards[4].value
    {
        return 2;
    }

    // check for one pair
    for i in 0..sorted_cards.len() - 1 {
        if sorted_cards[i].value == sorted_cards[i + 1].value {
            return 1;
        }
    }

    // high card
    return 0;
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hands = vec![];
    let mut max_rank: u8 = 0;
    for hand in hands {
        let cards: Vec<Card> = hand.split_whitespace().map(|s| Card::from_str(s)).collect();
        let rank = rank_cards(&cards);
        if rank > max_rank {
            max_rank = rank;
            winning_hands.clear();
            winning_hands.push(*hand);
        } else if rank == max_rank {
            winning_hands.push(*hand);
        }
    }
    winning_hands
}
