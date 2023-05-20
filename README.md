# Mao
A card games where you find out the rules as you go.

## Rules
These are the base rules and how things will work. If you don't want the rules and mechanics spoiled for you, then don't look further.

**PROGRESS FURTHER AT YOUR OWN RISK**

- Game starts with "Commence".
- Players are dealt 7 cards each.
- Players may play any card from their hand that is of the same suit or same value as top card in `used_pile`.
- If a player has no playable card, they must draw from the deck
- Player receives a card for breaking a rule - that player must say “thank you”
- If player doesn't say what they are supposed to say, then game says "Failure to say \_." That is also a penalty, and they would be given a card.
  - Probably have the system wait for 5 seconds before saying "Failure to say \_.
  - Same thing if they say something different that is not a different message. (eg. "bob ross")
- If player says something they can say in another situation, but they do it in the wrong situtation, game says, "Incorrect use of \_."
- If player plays a 7, they must say "Have a nice day." If they don't, the game says "Failure to say 'Have a nice day.'" and would penalize them.
- For each additional 7 played on top of the base 7, the next player must add a “very” before the “nice”.
  - For example, if it is the fourth 7 played, the player who placed that 7 must say “have a very, very, very nice day
  - Whoever breaks the chain or says it wrong would get a penalty equal to number of "very"s.
- If a player plays a spade, they need to say card value + "of spades."
  - For example, playing an ace of spades would require them to say "Ace of Spades."
- Playing an Ace skips the next player
- Playing a 2 allows you to go again
- Playing an 8 flips the direction of play
- Jack is a wild, allowing them to change the suit.
- If someone has 1 card, they have to say "Mao." If they don't, the system penalizes them with "Failure to say 'Mao.'" and gives them one card.
- The aim of the game is to get rid of all of your cards.
- Whoever wins a round can add or remove a rule for the next round.

## Gameplay Loop
1. Player can see their hand and top card of `used_pile`
2. Player chooses (0) to take a card or (1+) to play one of their cards
3. Player is prompted if they want to speak. They separate their phrases with periods. (*Perhaps disallow speaking rules with periods?*) (`say` or `speak`? I think `speak` is the best option) An example:
```
Speak?: 7 of spades. Have a nice day.
Say?: Have a nice day. 8 of spades.
> "Incorrect use of 'Have a nice day.'"
> Dealt 
... card ...
Speak?:
> "Failure to say 'Thank you.'"
> Dealt
... card ...
Speak?:
> "Failure to say, 'Thank you.'"
> Dealt 
... card ...
Speak?: .
> "Failure to say, 'Thank you.'"
> Dealt
... card ...
Speak?: thank you
```
  
