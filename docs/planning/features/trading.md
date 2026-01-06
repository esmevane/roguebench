# Trading

Exchange interface for transferring items and currency between entities.

## Core Logic

**Concept**

- Two parties exchange goods
- Can be player-NPC or player-player
- Mutual agreement required
- Atomic exchange

**Trade State**

| Property | Type | Description |
|----------|------|-------------|
| party_a | entity | First trader |
| party_b | entity | Second trader |
| offer_a | list | What A offers |
| offer_b | list | What B offers |
| confirmed_a | bool | A accepted |
| confirmed_b | bool | B accepted |

**Trade Offer**

- Items with quantities
- Currencies with amounts
- Mixed allowed

**Operations**

`initiate_trade(target)` - Start trade

- Create trade session
- Both parties see interface

`offer_item(item_id, quantity)` - Add to offer

- Add item to own offer
- Reset confirmations

`offer_currency(currency_id, amount)` - Add currency

- Add currency to offer
- Reset confirmations

`withdraw(item_or_currency)` - Remove from offer

- Remove from own offer
- Reset confirmations

`confirm()` - Accept current offers

- Mark self as confirmed
- If both confirmed, execute

`cancel()` - Abort trade

- Close session
- No exchange occurs

**Invariants**

- Both must confirm for exchange
- Modification resets confirmations
- Exchange atomic (all or nothing)
- Can only trade tradeable items

**Design Notes**

- NPC trade behavior left to design
- Trade restrictions left to design
- UI presentation left to design

---

## Bevy Integration

**Data**

- TradeSession { party_a, party_b, offers, confirmations }
- TradeOffer { items: Vec<(ItemId, u32)>, currencies: Vec<(CurrencyId, u64)> }

**Messages/Commands**

- InitiateTrade { target }
- OfferItem { item_id, quantity }
- OfferCurrency { currency_id, amount }
- WithdrawOffer { item_or_currency }
- ConfirmTrade
- CancelTrade

**Events**

- TradeInitiated { party_a, party_b }
- TradeOfferChanged { party, offer }
- TradeConfirmed { party }
- TradeCompleted { party_a, party_b, exchanged }
- TradeCancelled

**UI**

- Split view: your offer / their offer
- Confirm button
- Cancel button
- Tradeable item indicators

**Scripting Compatibility**

- Trade actions as commands
- Session state readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
