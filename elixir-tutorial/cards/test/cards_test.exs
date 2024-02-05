defmodule CardsTest do
  use ExUnit.Case
  doctest Cards

  test "create deck creates 16 cards" do
    deck_len = length(Cards.create_deck)
    assert deck_len == 16
  end

  test "shuffling a deck randomizes it" do
    deck = Cards.create_deck
    refute deck == Cards.shuffle(deck)
  end
end
